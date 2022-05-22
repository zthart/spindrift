# Spindrift

[![Dev Release](https://github.com/zthart/spindrift/actions/workflows/release.yml/badge.svg)](https://github.com/zthart/spindrift/actions/workflows/release.yml)

A really really simple static site generator focused on short posts (droplets) that feature an optional image and text-based content.

## Usage

Process all `.yaml` or `.yml` files in `<SOURCE_DIR>`, using templates in a `./templates` directory, writing the processed `.html` files to `<OUT_DIR>`:

```bash
./spindrift -s <SOURCE_DIR> -o <OUT_DIR>
```

Process all `.yaml` or `.yml` files in `<SOURCE_DIR>`, using templates in a specific `<TEMPLATE_DIR>`, writing the processed `.html` files to `<OUT_DIR>`:

```bash
./spindrift -s <SOURCE_DIR> -t <TEMPLATE_DIR> -o <OUT_DIR>
```

## Droplets

A droplet is spindrift's concept of a single "post".
Droplets are YAML files with only four top-level properties

- `title`
- `meta`
- `image` (optional)
- `content` (optional)

The `meta` and `image` properties are objects themselves

### `meta` property

The `meta` property contains metadata about the droplet, including the author, post date, and any user-provided tags for the post:

- the `author` property is required (in the future, this property should be configurable at the top-level spindrift configuration).
- the `tags` property is optional, and represents a list of tags that apply to the droplet. These tags can be used within meta tags in the HTML, or displayed within the post itself. In the future, the spindrift index will contain sub-indices or post counts for each tag.
- The `date` property is optional and corresponds to the date of the post. The droplet index page can be sorted by date if you prefer a chronological posting order. This property accepts a date in `YYYY-MM-DD` format.

### `image` property

The `image` property represents a single image associated with the given post, and contains three subproperties:

- The `src` property is required and should be a relative or absolute path to the image. If a relative path is used, it should be relative to the location of the droplet's .yaml file. If the `image` property is present at the top level of the yaml document, it must have a `src` subproperty.
- The `alt` property is optional. This property accepts a string or yaml block text and is intended to be used for alt text or a caption for the image.


### Sample droplet

```yaml
title: Example Drpolet
meta:
  author: John Q Spindrift
  tags:
    - sample
    - example
    - spindrift
  date: 2022-01-01 # YYYY-MM-DD format support only
image:
  src: ./images/some-image.png # path relative to this file, not from where the `spindrift` command will be run
  alt: >
    This is some alt text!
  copyright: John Q Rightsholder
content: >
  Here is where the main content of your post can go. Spindrift supports **bolded** text, _italicised_ text, and [links](https://google.com) in markdown format.

  Newlines in the content will be preserved in the rendered HTML, wrapped by a new <p /> tag.
```

## Templates

Sprindrift uses [tera](https://tera.netlify.app/) templating to place the parsed droplet files and spindrift configuration into the generated HTML.
Tera is similar to Jinja2, and supports much of the same syntax. Eventually, some defalut templates will be available.

Currently, spindrift expects a single `droplet.html` template within the provided templates directory (this defaults to `./templates`).
This template will be used to render each individual post/droplet to HTML.
These rendered droplets will be written to the directory provided via the `-o`/`--out-dir` argument.
In the future, an `spindrift.html` template will be used to render the post index.

### TODO:

- [ ] Default templates within the project source so that a run of the program without any templates still succeeds
- [ ] CSS support
- [ ] Project-level config so that properties like `author` can be set for all droplets
- [x] Markdown support in text content
- [ ] S3 or other storage backends for images
- [ ] Basic themeing framework for custom css
