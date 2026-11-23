; *** Inno Setup version 6.4.0+ Japanese messages ***

[LangOptions]
LanguageName=<65E5><672C><8A9E>
LanguageID=$0411
LanguageCodePage=932

[Messages]

; *** Application titles
SetupAppTitle=セットアップ
SetupWindowTitle=%1 セットアップ
UninstallAppTitle=アンインストール
UninstallAppFullTitle=%1 アンインストール

; *** Misc. common
InformationTitle=情報
ConfirmTitle=確認
ErrorTitle=エラー

; *** SetupLdr messages
SetupLdrStartupMessage=%1 をインストールします。続行しますか？
LdrCannotCreateTemp=一時ファイルを作成できませんでした。セットアップを中止します
LdrCannotExecTemp=一時ディレクトリ内のファイルを実行できませんでした。セットアップを中止します
HelpTextNote=

; *** Startup error messages
LastErrorMessage=%1.%n%nエラー %2: %3
SetupFileMissing=インストールディレクトリにファイル %1 が見つかりません。問題を修正するか、新しいプログラムを入手してください。
SetupFileCorrupt=セットアップファイルが破損しています。新しいプログラムを入手してください。
SetupFileCorruptOrWrongVer=セットアップファイルが破損しているか、このバージョンのセットアップと互換性がありません。問題を修正するか、新しいプログラムを入手してください。
InvalidParameter=コマンドラインに無効なパラメーターが渡されました:%n%n%1
SetupAlreadyRunning=セットアップは既に実行されています。
WindowsVersionNotSupported=このプログラムはお使いの Windows のバージョンをサポートしていません。
WindowsServicePackRequired=このプログラムには %1 Service Pack %2 以降が必要です。
NotOnThisPlatform=このプログラムは %1 上では動作しません。
OnlyOnThisPlatform=このプログラムは %1 上でのみ動作します。
OnlyOnTheseArchitectures=このプログラムは以下のプロセッサアーキテクチャ向けに設計された Windows のバージョンにのみインストールできます:%n%n%1
WinVersionTooLowError=このプログラムには %1 バージョン %2 以降が必要です。
WinVersionTooHighError=このプログラムは %1 バージョン %2 以降にはインストールできません。
AdminPrivilegesRequired=このプログラムをインストールするには管理者としてログオンする必要があります。
PowerUserPrivilegesRequired=このプログラムをインストールするには管理者または Power Users グループのメンバーとしてログオンする必要があります。
SetupAppRunningError=%1 が現在実行されていることが検出されました。%n%n今すぐすべてのインスタンスを閉じてから「OK」をクリックして続行するか、「キャンセル」をクリックして終了してください。
UninstallAppRunningError=%1 が現在実行されていることが検出されました。%n%n今すぐすべてのインスタンスを閉じてから「OK」をクリックして続行するか、「キャンセル」をクリックして終了してください。

; *** Startup questions
PrivilegesRequiredOverrideTitle=セットアップのインストールモードの選択
PrivilegesRequiredOverrideInstruction=インストールモードを選択してください
PrivilegesRequiredOverrideText1=%1 はすべてのユーザー向けにインストール（管理者権限が必要）することも、現在のユーザーのみにインストールすることもできます。
PrivilegesRequiredOverrideText2=%1 は現在のユーザーのみにインストールすることも、すべてのユーザー向けにインストール（管理者権限が必要）することもできます。
PrivilegesRequiredOverrideAllUsers=すべてのユーザー向けにインストール(&A)
PrivilegesRequiredOverrideAllUsersRecommended=すべてのユーザー向けにインストール(&A)（推奨）
PrivilegesRequiredOverrideCurrentUser=現在のユーザーのみにインストール(&M)
PrivilegesRequiredOverrideCurrentUserRecommended=現在のユーザーのみにインストール(&M)（推奨）

; *** Misc. errors
ErrorCreatingDir=ディレクトリ "%1" を作成できませんでした
ErrorTooManyFilesInDir=ディレクトリ "%1" 内のファイルが多すぎるため、ファイルを作成できませんでした

; *** Setup common messages
ExitSetupTitle=セットアップの終了
ExitSetupMessage=セットアップは完了していません。今終了すると、プログラムはインストールされません。%n%n後でもう一度セットアップを実行してインストールを完了できます。%n%nセットアップを終了しますか？
AboutSetupMenuItem=セットアップについて(&A)...
AboutSetupTitle=セットアップについて
AboutSetupMessage=%1 バージョン %2%n%3%n%n%1 ホームページ:%n%4
AboutSetupNote=
TranslatorNote=

; *** Buttons
ButtonBack=< 戻る(&B)
ButtonNext=次へ(&N) >
ButtonInstall=インストール(&I)
ButtonOK=OK
ButtonCancel=キャンセル
ButtonYes=はい(&Y)
ButtonYesToAll=すべてはい(&A)
ButtonNo=いいえ(&N)
ButtonNoToAll=すべていいえ(&O)
ButtonFinish=完了(&F)
ButtonBrowse=参照(&B)...
ButtonWizardBrowse=参照(&R)...
ButtonNewFolder=新しいフォルダーの作成(&M)

; *** "Select Language" dialog messages
SelectLanguageTitle=セットアップ言語の選択
SelectLanguageLabel=インストール中に使用する言語を選択してください。

; *** Common wizard text
ClickNext=続行するには「次へ」を、セットアップを終了するには「キャンセル」をクリックしてください。
BeveledLabel=
BrowseDialogTitle=フォルダーの参照
BrowseDialogLabel=下のリストからフォルダーを選択し、「OK」をクリックしてください。
NewFolderName=新しいフォルダー

; *** "Welcome" wizard page
WelcomeLabel1=[name] セットアップウィザードへようこそ
WelcomeLabel2=お使いのコンピューターに [name/ver] をインストールします。%n%n続行する前に、他のすべてのアプリケーションを閉じることを推奨します。

; *** "Password" wizard page
WizardPassword=パスワード
PasswordLabel1=このインストールはパスワードで保護されています。
PasswordLabel3=パスワードを入力し、「次へ」をクリックして続行してください。パスワードは大文字と小文字が区別されます。
PasswordEditLabel=パスワード(&P):
IncorrectPassword=入力されたパスワードが正しくありません。もう一度やり直してください。

; *** "License Agreement" wizard page
WizardLicense=使用許諾契約書
LicenseLabel=続行する前に、以下の重要な情報をお読みください。
LicenseLabel3=以下の使用許諾契約書をお読みください。インストールを続行するには、この契約の条項に同意する必要があります。
LicenseAccepted=同意する(&A)
LicenseNotAccepted=同意しない(&D)

; *** "Information" wizard pages
WizardInfoBefore=情報
InfoBeforeLabel=続行する前に、以下の重要な情報をお読みください。
InfoBeforeClickLabel=セットアップを続行する準備ができたら、「次へ」をクリックしてください。
WizardInfoAfter=情報
InfoAfterLabel=続行する前に、以下の重要な情報をお読みください。
InfoAfterClickLabel=セットアップを続行する準備ができたら、「次へ」をクリックしてください。

; *** "User Information" wizard page
WizardUserInfo=ユーザー情報
UserInfoDesc=あなたの情報を入力してください。
UserInfoName=ユーザー名(&U):
UserInfoOrg=組織名(&O):
UserInfoSerial=シリアル番号(&S):
UserInfoNameRequired=名前を入力する必要があります。

; *** "Select Destination Location" wizard page
WizardSelectDir=インストール先の指定
SelectDirDesc=[name] をどこにインストールしますか？
SelectDirLabel3=[name] を以下のフォルダーにインストールします。
SelectDirBrowseLabel=続行するには「次へ」をクリックしてください。別のフォルダーを選択するには「参照」をクリックしてください。
DiskSpaceGBLabel=少なくとも [gb] GB の空きディスク領域が必要です。
DiskSpaceMBLabel=少なくとも [mb] MB の空きディスク領域が必要です。
CannotInstallToNetworkDrive=ネットワークドライブにはインストールできません。
CannotInstallToUNCPath=UNC パスにはインストールできません。
InvalidPath=ドライブ文字を含む完全なパスを入力してください。例:%n%nC:\APP%n%nまたは次の形式の UNC パス:%n%n\\server\share
InvalidDrive=選択されたドライブまたは UNC 共有が存在しないかアクセスできません。別の場所を選択してください。
DiskSpaceWarningTitle=ディスク空き領域不足
DiskSpaceWarning=インストールには少なくとも %1 KB の空き領域が必要ですが、選択されたドライブには %2 KB しか空きがありません。%n%nそれでも続行しますか？
DirNameTooLong=フォルダー名またはパスが長すぎます。
InvalidDirName=フォルダー名が有効ではありません。
BadDirName32=フォルダー名には次の文字を含めることはできません:%n%n%1
DirExistsTitle=フォルダーが既に存在します
DirExists=フォルダー:%n%n%1%n%nは既に存在します。それでもこのフォルダーにインストールしますか？
DirDoesntExistTitle=フォルダーが存在しません
DirDoesntExist=フォルダー:%n%n%1%n%nは存在しません。このフォルダーを作成しますか？

; *** "Select Components" wizard page
WizardSelectComponents=コンポーネントの選択
SelectComponentsDesc=どのコンポーネントをインストールしますか？
SelectComponentsLabel2=インストールするコンポーネントを選択し、インストールしないコンポーネントのチェックを外してから、「次へ」をクリックしてください。
FullInstallation=フルインストール
CompactInstallation=コンパクトインストール
CustomInstallation=カスタムインストール
NoUninstallWarningTitle=コンポーネントが存在します
NoUninstallWarning=以下のコンポーネントが既にコンピューターにインストールされていることが検出されました:%n%n%1%n%nこれらのコンポーネントの選択を解除してもアンインストールはされません。%n%nそれでも続行しますか？
ComponentSize1=%1 KB
ComponentSize2=%1 MB
ComponentsDiskSpaceGBLabel=現在の選択には少なくとも [gb] GB のディスク領域が必要です。
ComponentsDiskSpaceMBLabel=現在の選択には少なくとも [mb] MB のディスク領域が必要です。

; *** "Select Additional Tasks" wizard page
WizardSelectTasks=追加タスクの選択
SelectTasksDesc=どの追加タスクを実行しますか？
SelectTasksLabel2=[name] のインストール中にセットアップが実行する追加タスクを選択し、「次へ」をクリックしてください。

; *** "Select Start Menu Folder" wizard page
WizardSelectProgramGroup=スタートメニューフォルダーの指定
SelectStartMenuFolderDesc=プログラムのショートカットをどこに作成しますか？
SelectStartMenuFolderLabel3=以下のスタートメニューフォルダーにプログラムのショートカットを作成します。
SelectStartMenuFolderBrowseLabel=続行するには「次へ」をクリックしてください。別のフォルダーを選択するには「参照」をクリックしてください。
MustEnterGroupName=フォルダー名を入力する必要があります。
GroupNameTooLong=フォルダー名またはパスが長すぎます。
InvalidGroupName=フォルダー名が有効ではありません。
BadGroupName=フォルダー名には次の文字を含めることはできません:%n%n%1
NoProgramGroupCheck2=スタートメニューフォルダーを作成しない(&D)

; *** "Ready to Install" wizard page
WizardReady=インストール準備完了
ReadyLabel1=お使いのコンピューターに [name] をインストールする準備ができました。
ReadyLabel2a=インストールを続行するには「インストール」をクリックしてください。設定を確認または変更する場合は「戻る」をクリックしてください。
ReadyLabel2b=インストールを続行するには「インストール」をクリックしてください。
ReadyMemoUserInfo=ユーザー情報:
ReadyMemoDir=インストール先:
ReadyMemoType=セットアップの種類:
ReadyMemoComponents=選択されたコンポーネント:
ReadyMemoGroup=スタートメニューフォルダー:
ReadyMemoTasks=追加タスク:

; *** TExtractionWizardPage wizard page and Extract7ZipArchive
ExtractionLabel=追加ファイルを展開しています...
ButtonStopExtraction=展開を停止(&S)
StopExtraction=展開を停止してもよろしいですか？
ErrorExtractionAborted=展開が中止されました
ErrorExtractionFailed=展開に失敗しました: %1

; *** TDownloadWizardPage wizard page and DownloadTemporaryFile
DownloadingLabel=追加ファイルをダウンロードしています...
ButtonStopDownload=ダウンロードを停止(&S)
StopDownload=ダウンロードを停止してもよろしいですか？
ErrorDownloadAborted=ダウンロードが中止されました
ErrorDownloadFailed=ダウンロードに失敗しました: %1 %2
ErrorDownloadSizeFailed=ダウンロードサイズの取得に失敗しました: %1 %2
ErrorFileHash1=ファイルハッシュの確認に失敗しました: %1
ErrorFileHash2=無効なファイルハッシュ: 期待値 %1、実際の値 %2
ErrorProgress=無効な進捗: %1 / %2
ErrorFileSize=無効なファイルサイズ: 期待値 %1、実際の値 %2

; *** "Preparing to Install" wizard page
WizardPreparing=インストールの準備中
PreparingDesc=お使いのコンピューターに [name] をインストールする準備をしています。
PreviousInstallNotCompleted=以前のプログラムのインストールまたは削除が完了していません。そのセットアップを完了するにはコンピューターを再起動する必要があります。%n%nコンピューターを再起動した後、もう一度セットアップを実行して [name] のインストールを完了してください。
CannotContinue=セットアップを続行できません。「キャンセル」をクリックして終了してください。
ApplicationsFound=以下のアプリケーションが、セットアップによって更新される必要のあるファイルを使用しています。セットアップがこれらのアプリケーションを自動的に閉じることを許可することを推奨します。
ApplicationsFound2=以下のアプリケーションが、セットアップによって更新される必要のあるファイルを使用しています。セットアップがこれらのアプリケーションを自動的に閉じることを許可することを推奨します。インストールが完了した後、セットアップはアプリケーションの再起動を試みます。
CloseApplications=アプリケーションを自動的に閉じる(&A)
DontCloseApplications=アプリケーションを閉じない(&D)
ErrorCloseApplications=すべてのアプリケーションを自動的に閉じることができませんでした。続行する前に、セットアップによって更新される必要のあるファイルを使用しているすべてのアプリケーションを閉じることを推奨します。
PrepareToInstallNeedsRestart=セットアップはコンピューターを再起動する必要があります。コンピューターを再起動した後、もう一度セットアップを実行して [name] のインストールを完了してください。%n%n今すぐ再起動しますか？

; *** "Installing" wizard page
WizardInstalling=インストール中
InstallingLabel=[name] をお使いのコンピューターにインストールしています。しばらくお待ちください。

; *** "Setup Completed" wizard page
FinishedHeadingLabel=[name] セットアップウィザードの完了
FinishedLabelNoIcons=お使いのコンピューターへの [name] のインストールが完了しました。
FinishedLabel=お使いのコンピューターへの [name] のインストールが完了しました。インストールされたショートカットを選択してアプリケーションを起動できます。
ClickFinish=セットアップを終了するには「完了」をクリックしてください。
FinishedRestartLabel=[name] のインストールを完了するには、コンピューターを再起動する必要があります。今すぐ再起動しますか？
FinishedRestartMessage=[name] のインストールを完了するには、コンピューターを再起動する必要があります。%n%n今すぐ再起動しますか？
ShowReadmeCheck=はい、README ファイルを表示します
YesRadio=はい、今すぐコンピューターを再起動します(&Y)
NoRadio=いいえ、後でコンピューターを再起動します(&N)
RunEntryExec=%1 を実行する
RunEntryShellExec=%1 を表示する

; *** "Setup Needs the Next Disk" stuff
ChangeDiskTitle=次のディスクが必要です
SelectDiskLabel2=ディスク %1 を挿入して「OK」をクリックしてください。%n%nこのディスクのファイルが下に表示されているフォルダーとは別の場所にある場合は、正しいパスを入力するか「参照」をクリックしてください。
PathLabel=パス(&P):
FileNotInDir2=ファイル "%1" は "%2" に見つかりませんでした。正しいディスクを挿入するか、別のフォルダーを選択してください。
SelectDirectoryLabel=次のディスクの場所を指定してください。

; *** Installation phase messages
SetupAborted=セットアップは完了しませんでした。%n%n問題を修正してから、もう一度セットアップを実行してください。
AbortRetryIgnoreSelectAction=操作を選択してください
AbortRetryIgnoreRetry=再試行(&T)
AbortRetryIgnoreIgnore=エラーを無視して続行(&I)
AbortRetryIgnoreCancel=インストールをキャンセル

; *** Installation status messages
StatusClosingApplications=アプリケーションを閉じています...
StatusCreateDirs=ディレクトリを作成しています...
StatusExtractFiles=ファイルを展開しています...
StatusCreateIcons=ショートカットを作成しています...
StatusCreateIniEntries=INI エントリを作成しています...
StatusCreateRegistryEntries=レジストリエントリを作成しています...
StatusRegisterFiles=ファイルを登録しています...
StatusSavingUninstall=アンインストール情報を保存しています...
StatusRunProgram=インストールを完了しています...
StatusRestartingApplications=アプリケーションを再起動しています...
StatusRollback=変更をロールバックしています...

; *** Misc. errors
ErrorInternal2=内部エラー: %1
ErrorFunctionFailedNoCode=%1 に失敗しました
ErrorFunctionFailed=%1 に失敗しました; コード %2
ErrorFunctionFailedWithMessage=%1 に失敗しました; コード %2.%n%3
ErrorExecutingProgram=ファイルを実行できません:%n%1

; *** Registry errors
ErrorRegOpenKey=レジストリキーを開く際にエラーが発生しました:%n%1\%2
ErrorRegCreateKey=レジストリキーを作成する際にエラーが発生しました:%n%1\%2
ErrorRegWriteKey=レジストリキーへ書き込む際にエラーが発生しました:%n%1\%2

; *** INI errors
ErrorIniEntry=ファイル "%1" に INI エントリを作成する際にエラーが発生しました。

; *** File copying errors
FileAbortRetryIgnoreSkipNotRecommended=このファイルをスキップする(&S)（非推奨）
FileAbortRetryIgnoreIgnoreNotRecommended=エラーを無視して続行(&I)（非推奨）
SourceIsCorrupted=ソースファイルが破損しています
SourceDoesntExist=ソースファイル "%1" が存在しません
ExistingFileReadOnly2=既存のファイルが読み取り専用としてマークされているため、置き換えることができません。
ExistingFileReadOnlyRetry=読み取り専用属性を解除して再試行(&R)
ExistingFileReadOnlyKeepExisting=既存のファイルを保持する(&K)
ErrorReadingExistingDest=既存のファイルを読み取る際にエラーが発生しました:
FileExistsSelectAction=操作を選択してください
FileExists2=ファイルは既に存在します。
FileExistsOverwriteExisting=既存のファイルを上書きする(&O)
FileExistsKeepExisting=既存のファイルを保持する(&K)
FileExistsOverwriteOrKeepAll=次の競合にも同じ操作を適用する(&D)
ExistingFileNewerSelectAction=操作を選択してください
ExistingFileNewer2=既存のファイルは、セットアップがインストールしようとしているファイルよりも新しいバージョンです。
ExistingFileNewerOverwriteExisting=既存のファイルを上書きする(&O)
ExistingFileNewerKeepExisting=既存のファイルを保持する(&K)（推奨）
ExistingFileNewerOverwriteOrKeepAll=次の競合にも同じ操作を適用する(&D)
ErrorChangingAttr=既存のファイルの属性を変更する際にエラーが発生しました:
ErrorCreatingTemp=インストール先ディレクトリにファイルを作成する際にエラーが発生しました:
ErrorReadingSource=ソースファイルを読み取る際にエラーが発生しました:
ErrorCopying=ファイルをコピーする際にエラーが発生しました:
ErrorReplacingExistingFile=既存のファイルを置き換える際にエラーが発生しました:
ErrorRestartReplace=RestartReplace に失敗しました:
ErrorRenamingTemp=インストール先ディレクトリ内のファイル名を変更する際にエラーが発生しました:
ErrorRegisterServer=DLL/OCX を登録できません: %1
ErrorRegSvr32Failed=RegSvr32 が終了コード %1 で失敗しました
ErrorRegisterTypeLib=タイプライブラリを登録できません: %1

; *** Uninstallation display name markings
UninstallDisplayNameMark=%1 (%2)
UninstallDisplayNameMarks=%1 (%2, %3)
UninstallDisplayNameMark32Bit=32ビット
UninstallDisplayNameMark64Bit=64ビット
UninstallDisplayNameMarkAllUsers=すべてのユーザー
UninstallDisplayNameMarkCurrentUser=現在のユーザー

; *** Post-installation errors
ErrorOpeningReadme=README ファイルを開く際にエラーが発生しました。
ErrorRestartingComputer=コンピューターを再起動できませんでした。手動で再起動してください。

; *** Uninstaller messages
UninstallNotFound=ファイル "%1" が存在しません。アンインストールできません。
UninstallOpenError=ファイル "%1" を開けませんでした。アンインストールできません
UninstallUnsupportedVer=アンインストールログファイル "%1" はこのバージョンのアンインストーラーでは認識できない形式です。アンインストールできません
UninstallUnknownEntry=アンインストールログに不明なエントリ (%1) が見つかりました
ConfirmUninstall=%1 とそのすべてのコンポーネントを完全に削除してもよろしいですか？
UninstallOnlyOnWin64=このインストールは 64 ビット Windows 上でのみアンインストールできます。
OnlyAdminCanUninstall=このインストールは管理者権限を持つユーザーのみがアンインストールできます。
UninstallStatusLabel=%1 をお使いのコンピューターから削除しています。しばらくお待ちください。
UninstalledAll=%1 はお使いのコンピューターから正常に削除されました。
UninstalledMost=%1 のアンインストールが完了しました。%n%n一部の要素を削除できませんでした。これらは手動で削除できます。
UninstalledAndNeedsRestart=%1 のアンインストールを完了するには、コンピューターを再起動する必要があります。%n%n今すぐ再起動しますか？
UninstallDataCorrupted=ファイル "%1" が破損しています。アンインストールできません

; *** Uninstallation phase messages
ConfirmDeleteSharedFileTitle=共有ファイルを削除しますか？
ConfirmDeleteSharedFile2=以下の共有ファイルは他のどのプログラムからも使用されていないことが示されています。この共有ファイルを削除しますか？%n%n他のプログラムがこのファイルを使用している場合に削除すると、それらのプログラムが正しく動作しなくなる可能性があります。不明な場合は「いいえ」を選択してください。ファイルをシステムに残しても害はありません。
SharedFileNameLabel=ファイル名:
SharedFileLocationLabel=場所:
WizardUninstalling=アンインストール状態
StatusUninstalling=%1 をアンインストールしています...

; *** Shutdown block reasons
ShutdownBlockReasonInstallingApp=%1 をインストールしています。
ShutdownBlockReasonUninstallingApp=%1 をアンインストールしています。

[CustomMessages]
NameAndVersion=%1 バージョン %2
AdditionalIcons=追加のショートカット:
CreateDesktopIcon=デスクトップ上にアイコンを作成する(&D)
CreateQuickLaunchIcon=クイック起動アイコンを作成する(&Q)
ProgramOnTheWeb=%1 ウェブサイト
UninstallProgram=%1 をアンインストール
LaunchProgram=%1 を起動する
AssocFileExtension=%1 を拡張子 %2 のファイルに関連付ける(&A)
AssocingFileExtension=%1 を拡張子 %2 のファイルに関連付けています...
AutoStartProgramGroupDescription=スタートアップ:
AutoStartProgram=%1 を自動的に起動する
AddonHostProgramNotFound=%1 は選択したフォルダーに見つかりませんでした。%n%nそれでも続行しますか？
