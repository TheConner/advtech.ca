use comrak::format_html;
use comrak::nodes::NodeValue;
use comrak::parse_document;
use comrak::Arena;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use walkdir::DirEntry;
use walkdir::WalkDir;

use comrak::Options;

use crate::config::PostMetadata;
use crate::templates::TEMPLATES;
use crate::util::copy_dir_all;
use crate::util::gen_cache_buster;
use crate::website::Website;

fn get_renderable_files(base_dir: &Path) -> impl Iterator<Item = DirEntry> {
    WalkDir::new(base_dir)
        .into_iter()
        .map(|result| result.unwrap())
        .filter(|entry| {
            entry.path().is_file() && entry.path().extension().is_some_and(|e| e.eq("md"))
        })
}

pub fn clear_output() -> io::Result<()> {
    fs::remove_dir_all(Path::new("./output"))
}

pub fn render_content(base_dir: &Path, website: &Website) -> Vec<PostMetadata> {
    let posts = get_renderable_files(base_dir).map(|x| Post::new(x));

    let mut post_metadatas = vec![];
    for mut post in posts {
        post.render(&website, false).expect("Could not render post");
        if let Some(meta) = post.metadata {
            post_metadatas.push(meta);
        }
    }
    post_metadatas
}

struct SplitPost {
    header: PostMetadata,
    body: String,
}

fn extract_header(text: &str) -> Option<SplitPost> {
    let re = Regex::new(r"(?ms)^---\n(?P<content>[\s\S]*?)\n---\n(?P<remaining>.*)")
        .expect("You screwed up the regex");

    if let Some(captures) = re.captures(text) {
        if let Some(content) = captures.name("content") {
            let meta: Result<PostMetadata, toml::de::Error> = toml::from_str(content.as_str());
            let remaining = String::from(captures.name("remaining").unwrap().as_str());
            let ret = SplitPost {
                header: meta.unwrap(),
                body: remaining,
            };
            return Some(ret);
        }
    }

    None
}

struct Post {
    file: DirEntry,
    assets: HashMap<PathBuf, PathBuf>,
    pub metadata: Option<PostMetadata>,
}

impl Post {
    fn new(file: DirEntry) -> Post {
        Post {
            file: file,
            metadata: None,
            assets: HashMap::new(),
        }
    }

    fn add_asset(&mut self, path: &PathBuf) {
        let mut src_path = path.clone();
        
        // Determine output path
        let out_path = gen_cache_buster(&mut src_path);
        let out_path = out_path.clone();

        // Map src to dest
        self.assets.insert(path.clone(), out_path);
    }

    fn render_md(&mut self, document: &str) -> String {
        let arena = Arena::new();

        let mut options = Options::default();
        options.render.unsafe_ = true; // I like to live dangerously
        options.extension.footnotes = true;

        let root = parse_document(&arena, document, &options);

        for node in root.descendants() {
            if let NodeValue::Image(ref mut img) = node.data.borrow_mut().value {
                // TODO: external vs internal images
                let path = Path::new(&img.url);
                let mut src_path_buf = path.to_path_buf();
                
                self.add_asset(&mut src_path_buf);

                let out_path = self.assets.get(&src_path_buf).unwrap();
                img.url = out_path.clone().into_os_string().into_string().unwrap();
            }
        }

        let mut html = vec![];
        format_html(root, &options, &mut html).unwrap();

        String::from_utf8(html).unwrap()
    }

    /// Renders a post
    /// TODO: make include_draft an options struct
    pub fn render(
        &mut self,
        website: &Website,
        include_draft: bool,
    ) -> std::io::Result<()> {
        let path = self.file.path().display();

        println!("-> render post {path}");
        let file_content = fs::read_to_string(self.file.path()).expect("Could not read file!");

        // File content at this point has a header
        // Extract the content header and split the file in to two parts
        let split_post = extract_header(file_content.as_str()).expect("Post metadata is borked");

        let metadata = split_post.header;

        // If this post is a draft, only continue if include_draft is set
        if metadata.is_draft() && !include_draft {
            println!("--> draft mode, skip");
            return Ok(());
        }

        if let Some(assets) = &metadata.assets {
            let asset_paths: Vec<PathBuf> = assets
                .into_iter()
                .map(|a| PathBuf::from_str(a.as_str()).unwrap())
                .collect::<Vec<PathBuf>>();

            for mut ele in asset_paths {
                self.add_asset(&mut ele);
            }
        }

        let html = self.render_md(&split_post.body);
        let mut context = website.render_context();

        context.insert("content", &html);
        context.insert("title", &metadata.title);
        context.insert("tags", &metadata.tags);
        context.insert("metadata", &metadata);

        let rendered_post = TEMPLATES.render("post.html", &context).unwrap();

        // Output html
        let base_output_path = Path::new("./output");
        let post_slug = Path::new(metadata.slug.as_str());
        let dir_path = base_output_path.join(post_slug);
        fs::create_dir_all(dir_path.clone()).expect("Fuck");

        let mut post_out = File::create(dir_path.join(Path::new("index.html")))?;
        post_out.write_all(rendered_post.as_bytes())?;

        // Output assets
        for (asset_src_path, asset_dest_path) in &self.assets {
            let asset_src = self.file.path().parent().unwrap().join(asset_src_path);
            let asset_src_str = asset_src.to_str().unwrap_or("");

            let asset_path = dir_path.join(asset_dest_path);
            let asset_dest_str = asset_path.to_str().unwrap_or("");
            println!("--> Copy {asset_src_str} to {asset_dest_str}");
            fs::copy(asset_src, asset_path).expect("Error copying asset!");
        }

        self.metadata = Some(metadata);

        Ok(())
    }
}

pub fn render_tags(website: &Website, posts: Vec<PostMetadata>) -> io::Result<()> {
    // Reduce all tags into a map of tag -> [post]
    let mut tag_map: HashMap<String, Vec<PostMetadata>> = HashMap::new();

    for ele in posts {
        let cl_meta = ele.clone();
        for tag in ele.tags {
            match tag_map.get_mut(&tag) {
                Some(vec) => {
                    vec.push(cl_meta.clone());
                },
                None => {
                    tag_map.insert(tag, Vec::from([cl_meta.clone()]));
                }
            }
        }
    }

    for (tag, posts) in tag_map {
        let mut context = website.render_context();
        context.insert("posts", &posts);
        context.insert("tag", &tag);
        let rendered = TEMPLATES.render("tags.html", &context).unwrap();
        let base_output_path_str = format!("output/tags/{tag}");
        let base_output_path = Path::new(&base_output_path_str);
        fs::create_dir_all(base_output_path).expect("Could not create base output path");
        let mut out_file = File::create(base_output_path.join(Path::new("index.html")))?;
        out_file.write_all(rendered.as_bytes())?;
    }

    Ok(())
}

pub fn render_index(website: &Website, posts: Vec<PostMetadata>) -> io::Result<()> {
    let mut context = website.render_context();
    context.insert("posts", &posts);
    let rendered = TEMPLATES.render("home.html", &context).unwrap();
    let base_output_path = Path::new("./output");
    fs::create_dir_all(base_output_path).expect("Could not create base output path");
    let mut out_file = File::create(base_output_path.join(Path::new("index.html")))?;
    out_file.write_all(rendered.as_bytes())?;

    Ok(())
}

pub fn render_styles() -> io::Result<()> {
    let scss = grass::from_path("./styles/theme.scss", &grass::Options::default())
        .expect("styles are fucked");
    let out_path = Path::new("./output/css");
    fs::create_dir_all(out_path).expect("Could not create style output");

    // TODO: make this less jank
    copy_dir_all("./styles/font/glasstty", "./output/font")
        .expect("Copying rendered styles blew up");
    copy_dir_all("./styles/font/ibm_plex_serif", "./output/font")
        .expect("Copying rendered styles blew up");

    let mut out_file = File::create(out_path.join(Path::new("main.css")))?;
    out_file.write_all(scss.as_bytes())?;

    Ok(())
}
