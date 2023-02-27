<h1 align="center">GIMM</h1>

English | [简体中文](README_zh-CN.md)

## Description

GIMM is a mod manager for an anime game, designed to conveniently manage all local mods. It can automatic categorization, manage custom tags, filter by keyword, edit metadata. It also provides practical features such as change outline thickness, merge specific mods, convert between DDS and PNG, and generate template INI files. However, the project is currently in early stages, so please be patient for the addition of more features.

## Download

Please go [here](https://github.com/jianxingxuejian/GIMM/releases) to download the latest version.
Alternatively,you can click on the latest `build` workflow in [Action](https://github.com/jianxingxuejian/GIMM/actions), and at the bottom of the page, you can obtain the zip file that contains the latest files compiled for every commit or pull request before the release.

## Contribution

The implementation of "automatic categorization" comes from this [repository](https://github.com/jianxingxuejian/GIMM-Assets). Please contribute by simply adding the name of an ini file. Currently, most of the data such as characters and weapons have been added, so the tool can accurately categorize these mods. For data that does not exist, it can only be classified as "other" and requires manual selection by the user.

If you would like to contribute to the translation, please go to the `src/i18n` folder to improve, optimize existing text, fix text errors, add new language support, etc.

## Development

Build by Tauri, with the front-end technology stack: Vue3 + Vite4 + Typescript + Naive-ui + Unocss + Vue I18n, and Rust for the back-end. Development requires **Node** and **Rust** environments, and package manager use **pnpm**.
Recommend using Visual Studio Code for development and installing recommended plugins.

Install dependencies:

```shell
pnpm install
```

Run：

```shell
pnpm tauri dev
```

Please go to the [Tauri](https://github.com/tauri-apps/tauri) project for more information.

## License

This project is licensed under the [AGPL-3.0](https://github.com/jianxingxuejian/GIMM/blob/main/LICENSE)
