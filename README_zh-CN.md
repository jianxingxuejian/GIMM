<h1 align="center">GIMM</h1>

[English](README.md) | 简体中文

## 说明

某个游戏的 mod 管理器，用于方便地管理所有本地 mod，可以对 mod 进行自动分类、管理自定义标签、根据关键词筛选、编辑元数据等，准备提供如更改轮廓线宽度、合并特定 mod、dds 与 png 的互相转换、模板 ini 生成等实用功能。不过现在项目处于刚刚起步的阶段，请耐心等待功能的添加。

## 下载

请前往[此处](https://github.com/jianxingxuejian/GIMM/releases)下载最新版本。
或者在[Action](https://github.com/jianxingxuejian/GIMM/actions)中点击最近一次的`build`工作流，在页面最下方获取 zip 文件，这是版本发布前每一次提交或者 PR 所编译的最新文件。

## 贡献

“自动分类”的实现来源于[这个仓库](https://github.com/jianxingxuejian/GIMM-Assets)，请帮忙贡献，非常简单，只要添加 ini 的名字就行，目前有大部分人物、武器等数据，因此当你使用工具时，mod 能准确为这部分 mod 进行分类，而不存在的数据只能归类为“其他”，等待用户手动去选择。

如果你想参与翻译的贡献，请前往 **src/i18n**文件夹，完善、优化现有文案，修复文案错误，添加新语言支持等。

## 开发

使用 Tauri 构建，前端技术栈：Vue3 + Vite4 + Typescript + Naive-ui + Unocss + Vue I18n，后端使用 Rust。开发需要 **Node** 和 **Rust** 环境，包管理使用 **pnpm**。
推荐使用 Visual Studio Code 进行开发并安装建议使用的插件。

安装依赖：

```shell
// 安装依赖之前需要全局安装pnpm，使用npm -g install pnpm进行安装
pnpm install
```

启动和打包：

```shell
pnpm tauri dev
pnpm tauri build or pnpm tauri build --debug
```

更详细的内容请前往 [Tauri](https://github.com/tauri-apps/tauri) 项目进行了解。

## 许可

本项目使用 [AGPL-3.0](https://github.com/jianxingxuejian/GIMM/blob/main/LICENSE) 许可证书。
