# Mysql Client

**I do not recommend using this software.**  
**`Cassin01/mysql-client` is under development.**

<!-- ref https://github.com/DenverCoder1/custom-icon-badges -->
<a href="https://tauri.app">
<img src="https://custom-icon-badges.demolab.com/badge/tauri-blue.svg?logo=tauri&logoColor=yellow">
</a>
<a href="https://yew.rs">
<img src="https://custom-icon-badges.demolab.com/badge/yew-brightgreen.svg?logo=yew&logoColor=white">
</a>
<a href="https://getbootstrap.com">
<img src="https://custom-icon-badges.demolab.com/badge/bootstrap-purple.svg?logo=bootstrap&logoColor=white">
</a>

## Usage
<details>
Here, the same directory with README.md:

```sh
$ cargo tauri dev
```
</details>

<h2> Setting </h2>
<details>
The configuration file will be automatically generated on:

- Linux: `~/.config/c01_mysql_client`
- Windows: `{FOLDERID_RoamingAppData}\c01_mysql_client`
- Mac OS: `~/Library/Preferences/rs.c01_mysql_client`

<details>
<summary>toml</summary>

```toml
user = 'user name'       # e.g. 'root'
pass = 'password'
dbname = 'database name' # e.g. 'my_schema'
protocol = 'protocol'    # e.g. 'localhost:3306'
dbms = 'mysql'           # e.g. 'mysql'
```

</details>
</details>


## Reference
<details>
- [Dev](https://dev.to/stevepryde/create-a-desktop-app-in-rust-using-tauri-and-yew-2bhe)
</details>

---

## Todo

<details>
- [ ] connectの更新イベントを起こす`use` or `event (such as onclick)`
- [x] テーブルの追加
    - [x] テーブルの選択
    - [ ] ポップアップにする
    - [ ] テーブルの削除
- [ ] テーブルへの要素追加
    - [ ] テーブルの要素の削除
</details>

## Dev log
<details>
<details>
    <summary>テーブルの選択</summary>
<img src="https://github.com/Cassin01/mysql-client/blob/ee2744d13034dd6e7c41a4a85a9f23f160afe1a8/asset/02.png" width="400">
</details>

<details>
    <summary>接続状況の表示, テーブルの追加フォーム</summary>
<img src="https://github.com/Cassin01/mysql-client/blob/a4b29bd9bf0a8018b6a7fbb25ddf8ce66f0f2fc2/asset/01.png" width="400">
</details>
</details>
