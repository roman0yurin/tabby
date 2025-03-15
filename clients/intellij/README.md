# Tabby Plugin for IntelliJ Platform

[![JetBrains plugins](https://img.shields.io/jetbrains/plugin/d/22379-tabby)](https://plugins.jetbrains.com/plugin/22379-tabby)
[![Slack Community](https://shields.io/badge/Tabby-Join%20Slack-red?logo=slack)](https://links.tabbyml.com/join-slack)

Tabby is an AI coding assistant that can suggest multi-line code or full functions in real-time.

Tabby IntelliJ Platform plugin works with all [IntelliJ Platform IDEs](https://plugins.jetbrains.com/docs/intellij/intellij-platform.html#ides-based-on-the-intellij-platform) that have build 2023.1 or later versions, such as [IDEA](https://www.jetbrains.com/idea/), [PyCharm](https://www.jetbrains.com/pycharm/), [GoLand](https://www.jetbrains.com/go/), [Android Studio](https://developer.android.com/studio), and [more](https://plugins.jetbrains.com/docs/intellij/intellij-platform.html#ides-based-on-the-intellij-platform).

## Getting Started

1. Set up the Tabby Server: you can build your self-hosted Tabby server following [this guide](https://tabby.tabbyml.com/docs/installation/).
2. Install Tabby plugin from [JetBrains Marketplace](https://plugins.jetbrains.com/plugin/22379-tabby).
3. Install [Node.js](https://nodejs.org/en/download/) version 18.0 or higher.
4. Open the settings by clicking on the Tabby plugin status bar item and select `Open Settings...`.
   1. Fill in the server endpoint URL to connect the plugin to your Tabby server.
   - If you are using default port `http://localhost:8080`, you can skip this step.
   2. If your Tabby server requires an authentication token, set your token in settings. Alternatively, you can set it in the [config file](https://tabby.tabbyml.com/docs/extensions/configurations).
   3. Enter the node binary path into the designated field
   - If node binary is already accessible via your `PATH` environment variable, you can skip this step.
   - Remember to save the settings and restart the IDE if you made changes to this option.
5. Check the Tabby plugin status bar item, it should display a check mark if the plugin is successfully connected to the Tabby server.

## Troubleshooting

If you encounter any problem, please check out our [troubleshooting guide](https://tabby.tabbyml.com/docs/extensions/troubleshooting).

## Development and Build

To develop and build Tabby plugin, please clone [this directory](https://github.com/TabbyML/tabby/tree/main/clients/intellij) and import it into IntelliJ Idea.

## Примечания по сборке плагина

### 1. Установить зависимости (видимо для внутренней JS части) 
Команды следует делать в корневой папке проекта
```shell
sudo    аnpm install -g pnpm
pnpm add -D turbo\n
pnpm install
```

### 2. Стандартные команды для сборки и тестирования
./gradlew <команда>
• buildPlugin  
Соберёт архив (zip) плагина в директории build/distributions.  
• runIde  
Запустит «песочницу» (sandbox) IntelliJ IDE вместе с вашим плагином.  
• publishPlugin  
Публикует плагин в Marketplace (при наличии соответствующих настроек).  
• test  
Запускает тесты (если в вашем проекте есть тесты для плагина).


### 3. Устранение возможных проблем

1. Если не работает ввод с клавиатуры в окно tabby чата, в частности может не работать кирилица.
Причина состоит в том что интерфейс tabby живет во встроенном браузере JCEF и этот браузер может некорректно работать 
из-за настройки переменных окружения, в частности способа ввода с клавитуры ibus.
Для исправления ошибки установите переменные окружения, можно в environment или через export
```bash
sudo nano /etc/environment

LANG=en_US.UTF-8
LC_ALL=en_US.UTF-8
XMODIFIERS="@im=fcitx"
GTK_IM_MODULE=fcitx
QT_IM_MODULE=fcitx
```


