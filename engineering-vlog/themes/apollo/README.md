# apollo

Modern and minimalistic blog theme powered by [Zola](https://getzola.org). See a live preview [here](https://not-matthias.github.io/apollo).

<sub><sup>Named after the greek god of knowledge, wisdom and intellect</sup></sub>

<details open>
  <summary>Dark theme</summary>

![blog-dark](./screenshot-dark.png)

</details>

<details>
  <summary>Light theme</summary>

![blog-light](./screenshot.png)

</details>

## Features

- [x] Pagination
- [x] Themes (light, dark, auto)
- [x] Projects page
- [x] Analytics using [GoatCounter](https://www.goatcounter.com/) / [Umami](https://umami.is/)
- [x] Social Links
- [x] MathJax Rendering
- [x] Taxonomies
- [x] Meta Tags For Individual Pages
- [x] Custom homepage
- [x] Comments
- [x] Search
- [x] RSS feeds
- [x] Mermaid diagram support
- [x] Table of Contents

## Installation

1. Download the theme

```
git submodule add https://github.com/not-matthias/apollo themes/apollo
```

2. Add the following to the top of your `config.toml`

```toml
theme = "apollo"
taxonomies = [{ name = "tags" }]

[extra]
theme = "auto"
socials = [
  # Configure socials here
]
menu = [
  # Configure menu bar here
]

# See this for more options: https://github.com/not-matthias/apollo/blob/main/config.toml#L14
```

3. Copy the example content

```
cp -r themes/apollo/content/* content/
```

## Configuration

Checkout all the [options you can configure](./content/posts/configuration.md) and the [example pages](./content/posts/).

## References

This theme is based on [archie-zola](https://github.com/XXXMrG/archie-zola/).
