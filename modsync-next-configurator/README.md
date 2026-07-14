# modsync-next-configurator

此项目是 Tauri 桌面应用。发布时请从仓库根目录执行：

```powershell
pnpm build:desktop
```

请使用 `target\\release\\bundle\\` 下的安装包，而不是 `cargo build` 或
`target\\debug` 的输出；后者是开发程序，会请求本地 Vite 服务
`http://localhost:1420`。
