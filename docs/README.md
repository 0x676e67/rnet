# rnet Documentation (MkDocs)

This directory contains the MkDocs-based documentation for rnet.

## Setup

Install dependencies:

```bash
pip install -r requirements.txt
```

## Development

Start the development server (from this directory):

```bash
mkdocs serve
```

Then open http://127.0.0.1:8000 in your browser.

## Build

Generate static HTML:

```bash
mkdocs build
```

The generated site will be in the `site/` directory.

## Deploy to GitHub Pages

```bash
mkdocs gh-deploy
```

This will build the docs and push them to the `gh-pages` branch.

## Structure

- `mkdocs.yml` - Main configuration file
- `index.md` - Home page
- `getting-started/` - Installation and quickstart guides
- `api/` - API reference (auto-generated from docstrings)
- `examples/` - Code examples and tutorials
