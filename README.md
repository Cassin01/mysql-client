<h1 align="center">C01 MySQL Client</h1>

<p align="center">MySQL client for daily use.</p>

<p align="center">
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
</p>

### ð§WARNð§

**I do not recommend using this software.**  
**`Cassin01/mysql-client` is under development.**

### Usage
<details>
Here, the same directory with README.md:

```sh
$ cargo tauri dev
```
</details>

### Setting
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


### Reference
<details>
- [Dev](https://dev.to/stevepryde/create-a-desktop-app-in-rust-using-tauri-and-yew-2bhe)
</details>

---

### Todo

<details>
- [ ] connectã®æ´æ°ã¤ãã³ããèµ·ãã`use` or `event (such as onclick)`
- [x] ãã¼ãã«ã®è¿½å 
    - [x] ãã¼ãã«ã®é¸æ
    - [ ] ãããã¢ããã«ãã
    - [ ] ãã¼ãã«ã®åé¤
- [ ] ãã¼ãã«ã¸ã®è¦ç´ è¿½å 
    - [ ] ãã¼ãã«ã®è¦ç´ ã®åé¤
</details>

### Dev log
<details>
<details>
    <summary>ãã¼ãã«ã®é¸æ</summary>
<img src="https://github.com/Cassin01/mysql-client/blob/ee2744d13034dd6e7c41a4a85a9f23f160afe1a8/asset/02.png" width="400">
</details>

<details>
    <summary>æ¥ç¶ç¶æ³ã®è¡¨ç¤º, ãã¼ãã«ã®è¿½å ãã©ã¼ã </summary>
<img src="https://github.com/Cassin01/mysql-client/blob/a4b29bd9bf0a8018b6a7fbb25ddf8ce66f0f2fc2/asset/01.png" width="400">
</details>
</details>
