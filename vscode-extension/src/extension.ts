import * as path from 'path';
import { workspace, ExtensionContext } from 'vscode';

import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    Executable,
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: ExtensionContext) {
    // LSP 服务器路径 (假设编译后的二进制在项目根目录的 target/release 或 target/debug)
    const serverExecutable: Executable = {
        command: path.join(
            context.extensionPath,
            '..',
            'target',
            'release',
            'aether-lsp'
        ),
        // 如果是开发模式,使用 debug 版本
        // command: path.join(context.extensionPath, '..', 'target', 'debug', 'aether-lsp'),
    };

    const serverOptions: ServerOptions = {
        run: serverExecutable,
        debug: serverExecutable,
    };

    // 客户端选项
    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'aether' }],
        synchronize: {
            fileEvents: workspace.createFileSystemWatcher('**/.aether'),
        },
    };

    // 创建 LSP 客户端
    client = new LanguageClient(
        'aetherLanguageServer',
        'Aether Language Server',
        serverOptions,
        clientOptions
    );

    // 启动客户端
    client.start();
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
