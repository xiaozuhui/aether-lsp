import * as path from 'path';
import * as fs from 'fs';
import { workspace, ExtensionContext, window } from 'vscode';

import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    Executable,
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: ExtensionContext) {
    // 确定二进制文件名（Windows 需要 .exe 后缀）
    const binaryName = process.platform === 'win32' ? 'aether-lsp.exe' : 'aether-lsp';
    
    // 尝试查找 LSP 服务器二进制文件
    const releasePath = path.join(context.extensionPath, '..', 'target', 'release', binaryName);
    const debugPath = path.join(context.extensionPath, '..', 'target', 'debug', binaryName);
    
    let serverCommand: string;
    
    // 优先使用 release 版本，否则使用 debug 版本
    if (fs.existsSync(releasePath)) {
        serverCommand = releasePath;
        console.log('[Aether LSP] 使用 release 版本:', serverCommand);
    } else if (fs.existsSync(debugPath)) {
        serverCommand = debugPath;
        console.log('[Aether LSP] 使用 debug 版本:', serverCommand);
    } else {
        window.showErrorMessage(
            'Aether LSP: 找不到服务器可执行文件。请先运行 `cargo build --release`'
        );
        console.error('[Aether LSP] 未找到二进制文件:', { releasePath, debugPath });
        return;
    }

    const serverExecutable: Executable = {
        command: serverCommand,
        options: {
            env: { ...process.env, RUST_LOG: 'info' },
        },
    };

    const serverOptions: ServerOptions = {
        run: serverExecutable,
        debug: {
            ...serverExecutable,
            options: {
                env: { ...process.env, RUST_LOG: 'debug' },
            },
        },
    };

    // 客户端选项
    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'aether' }],
        synchronize: {
            // 修复：正确监听 .aether 文件变化
            fileEvents: workspace.createFileSystemWatcher('**/*.aether'),
        },
    };

    // 创建 LSP 客户端
    client = new LanguageClient(
        'aetherLanguageServer',
        'Aether Language Server',
        serverOptions,
        clientOptions
    );

    // 启动客户端并处理错误
    client.start().catch((error) => {
        window.showErrorMessage(`Aether LSP 启动失败: ${error.message}`);
        console.error('[Aether LSP] 启动错误:', error);
    });

    console.log('[Aether LSP] 语言服务器已启动');
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
