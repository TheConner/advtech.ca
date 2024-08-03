---
title = "On Blogging"
slug = "posts/on-blogging"
date = "2024-07-16"
tags = ["meta"]
---

I have decided to revisit putting my musings out here. In the past, I found a variety of different meanings in writing about various topics and putting them out into the void for others to consume. It was primarily enjoyable as an exercise to learn and share information. 

The problem with this is that there's too much information out there already, and trying to be original is quite difficult. I'd rather contribute something if I'm actually saying something useful, and not echoing knowledge shared thousands of times already.

As a result, I am working on cleaning up my website. Most of my old posts have been archived, and I will actively work on porting them over to my new website engine. Speaking of which, I have moved off of Gatsby[^1] and onto my own static site rendering engine.

I don't really think I needed to write my own static site generator, I instead did it for fun, and now I don't have to worry about my engine being adopted by a company and development stagnating. If development stagnates on my own engine I only have myself to blame, which is fine.

My site generator is written in Rust, with sources [here](https://gitlab.advtech.ca/netwinder/advtech.ca) ([mirror](https://github.com/TheConner/advtech.ca)). It is really not that good, it's more of a pet project than a serious attempt at a good SSG. If you want a half decent SSG with good features, check out [Astro](https://astro.build/) or [Zola](https://www.getzola.org/). 


## Footnotes

[^1]: Due to Gatsby's acquisition by Netlify development has stagnated, which is quite unfortunate. I really liked parts of Gatsby as a static site generator.