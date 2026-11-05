# Fontes de jogos

> **O que é:** como cada loja ou launcher guarda os jogos instalados e como abri-los. É a base para escrever um adaptador novo.
> **Quando usar:** ao implementar ou revisar uma fonte. O contrato que cada fonte segue está em [ARQUITETURA.md](ARQUITETURA.md).
> **Estado:** levantamento feito no Windows. A coluna *Verificação* diz o que foi **conferido de fato** e o que ainda é **a verificar**; não trate "a verificar" como fato.

| Fonte | Como detectar | Como abrir | Sistemas | Verificação |
| --- | --- | --- | --- | --- |
| Steam | `steamapps/libraryfolders.vdf` lista as bibliotecas; cada uma tem `steamapps/appmanifest_<appid>.acf` com `name`, `installdir`, `SizeOnDisk` e `StateFlags` (o bit 4 indica instalação completa) | `steam://rungameid/<appid>` | Windows, Linux, macOS | Detecção implementada e conferida contra uma instalação real no Windows; a abertura (`steam://`) ainda não foi testada de ponta a ponta |
| Epic Games | `%ProgramData%\Epic\EpicGamesLauncher\Data\Manifests\*.item` (JSON em UTF-8, com `DisplayName` e `InstallLocation`) | `com.epicgames.launcher://apps/<AppName>?action=launch&silent=true` | Windows; macOS a verificar | Detecção conferida no Windows; abertura a verificar |
| GOG | Registro `HKLM\SOFTWARE\WOW6432Node\GOG.com\Games` e arquivos `goggame-<id>.info` na pasta do jogo | Executável do jogo | Windows | A verificar |
| Riot Games | `%ProgramData%\Riot Games\Metadata\` e a pasta do Riot Client | `RiotClientServices.exe --launch-product=<produto> --launch-patchline=live` | Windows | Pasta de metadados conferida; argumentos a verificar |
| Ubisoft Connect | Registro `HKLM\SOFTWARE\WOW6432Node\Ubisoft\Launcher\Installs` | `uplay://launch/<id>/0` | Windows | A verificar |
| Battle.net | `%ProgramData%\Battle.net\Agent\product.db` (formato binário) e entradas de desinstalação | `Battle.net.exe --exec="launch <produto>"` | Windows | A verificar |
| Rockstar | Registro `HKLM\SOFTWARE\WOW6432Node\Rockstar Games` e entradas de desinstalação com `InstallLocation` | Executável do jogo | Windows | Entradas de desinstalação conferidas |
| EA app | `%ProgramData%\EA Desktop` e entradas de desinstalação | `origin2://game/launch/?offerIds=<id>` | Windows | A verificar |
| Microsoft Store e Xbox | Pacotes instalados (`Get-AppxPackage`) e a pasta `XboxGames` | `shell:AppsFolder\<PackageFamilyName>!<AppId>` | Windows | A verificar |
| Minecraft Java | `%APPDATA%\.minecraft` e launchers de terceiros | Launcher escolhido | Windows, Linux, macOS | Caminho conferido no Windows |
| Roblox | `%LOCALAPPDATA%\Roblox\Versions\*\RobloxPlayerBeta.exe` | Executável | Windows | Conferido |
| osu! | `%LOCALAPPDATA%\osu!` (stable) e `%LOCALAPPDATA%\osulazer` (lazer) | Executável | Windows; lazer também em Linux e macOS | Conferido no Windows |
| Jogos avulsos | Pastas indicadas pelo usuário, atalhos do Menu Iniciar (`.lnk`), entradas de desinstalação do registro e marcadores de arquivo | Executável escolhido | Todos | Marcadores conferidos no Windows |
| Emuladores e ROMs | Pasta de ROMs mais o emulador associado | Linha de comando do emulador | Todos | A verificar |

## Marcadores de arquivo

Jogos instalados à mão não aparecem em nenhum launcher. A varredura profunda os acha por arquivos típicos:

- `UnityPlayer.dll` e `GameAssembly.dll` (Unity);
- `*-Shipping.exe` (Unreal Engine);
- `steam_api.dll` e `steam_api64.dll` (jogos com Steamworks);
- `goggame-*.info` (GOG).

Varrer vários discos com milhões de arquivos leva **minutos**. Por isso a varredura roda em segundo plano, com progresso visível, e a primeira execução é obrigatória.

## Armadilhas já medidas

- **Disco ausente ou com defeito.** As bibliotecas da Steam e o registro podem apontar para unidades que não estão montadas. O jogo aparece como *indisponível*; ele **nunca** é removido da biblioteca sozinho.
- **Restos de instalação.** Pastas em `steamapps/common` sem `appmanifest` correspondente existem e não são jogos instalados.
- **Caminhos.** No VDF a barra invertida vem escapada (`C:\\Steam`), e o registro do Windows guarda `c:/steam`. Compare pastas pelo caminho real, senão a mesma biblioteca é lida duas vezes.
- **Codificação.** Os manifestos da Epic são UTF-8; lê-los em outra codificação corrompe os acentos dos nomes.
- **Estado parcial.** O `StateFlags` da Steam pode indicar instalação completa com atualização pendente; o tamanho informado pode mudar depois da atualização.
- **Registro do Windows.** Só existe no Windows; em Linux e macOS cada fonte precisa do seu equivalente.
