<p align="center">
  <img src="https://acridotheres.com/favicon.png" height="128" alt="Acridotheres logo">
</p>
<h1 align="center">
  Acridotheres
</h1>
<p align="center"><i>
  The fast, modern, open-source and secure file archiver.<br>
  <sub>
    Created with <a href="https://github.com/rust-lang/rust#readme">Rust</a>, <a href="https://github.com/tauri-apps/tauri#readme">Tauri</a> and <a href="https://github.com/sveltejs/svelte#readme">Svelte</a>.
  </sub>
  <br><br>
  <img alt="GitHub License" src="https://img.shields.io/github/license/acridotheres/acridotheres">
  <img alt="GitHub Downloads (all assets, all releases)" src="https://img.shields.io/github/downloads/acridotheres/acridotheres/total">
  <img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/acridotheres/acridotheres?style=flat">
  <a href="https://ko-fi.com/le0_x8"><img alt="Donate" src="https://img.shields.io/badge/donate-080?logo=ko-fi&logoColor=fff"></a>
</i></p>

---

## Format support

Acridotheres currently supports 11 archive formats through the use of custom libraries. The following table lists all supported formats and their respective libraries.

| Format                                  | Internal name | Library                                                    |
| --------------------------------------- | ------------- | ---------------------------------------------------------- |
| HSSP 7                                  | `hssp7`       | [HSSP7](https://github.com/acridotheres/hssp7)             |
| ZIP                                     | `zip`         | [NeoZip](https://github.com/acridotheres/neozip)           |
| [RAR](#i-cant-create-a-rar-archive-why) | `rar`         | [NeoRar](https://github.com/acridotheres/neorar)           |
| HSSP 6                                  | `hssp6`       | [HSSP6](https://github.com/acridotheres/hssp6)             |
| HSSP 5                                  | `hssp5`       | [HSSP5](https://github.com/acridotheres/hssp5)             |
| HSSP 4                                  | `hssp4`       | [HSSP5](https://github.com/acridotheres/hssp5)             |
| UMSBT                                   | `umsbt`       | [3ds-formats](https://github.com/acridotheres/3ds-formats) |
| MSBT[\*](#-not-an-archive)              | `msbt`        | [3ds-formats](https://github.com/acridotheres/3ds-formats) |
| HSSP 3                                  | `hssp3`       | [HSSP2](https://github.com/acridotheres/hssp2)             |
| HSSP 2                                  | `hssp2`       | [HSSP2](https://github.com/acridotheres/hssp2)             |
| SFA (HSSP 1)                            | `hssp1`       | [HSSP2](https://github.com/acridotheres/hssp2)             |

Request a new format by [creating a new issue](https://github.com/acridotheres/core/issues/new?assignees=Le0X8&labels=format+request&projects=&template=format-request.md&title=Format+support%3A+%3CNAME%3E).

##### \*: Not an archive

Formats marked with an asterisk (\*) are not really archive formats, but Acridotheres still supports them :)

### Other internal format names

| Internal name | Description                                                                         |
| ------------- | ----------------------------------------------------------------------------------- |
| `auto`        | Automatically detects the archive format based on file extension, magic bytes, etc. |
| `hssp`        | Any HSSP version. Version is being detected automatically.                          |

## FAQ

### What is HSSP? Why is it special?

HSSP (High-Speed Secure Package) is Acridotheres' custom archiving format. It is more adaptable and furture-proof than any other of the supported archive types. In version 8, it will get rebranded to HIA, the HSSP Infinity Archive as this will be the last update to the file structure and literally will have no limits.

### I can't create a RAR archive, why?

RARlab's TOS don't allow other programs than WinRAR to create RAR archives, so we don't support this feature to avoid legal issues.

### What platforms are supported?

We currently only support Linux and Windows. Mobile versions are planned. MacOS is likely not going to be in development soon as I don't want to pay 800$ for a Mac Mini just to develop applications for this platform (this would be a massive downgrade from my Arch Linux PC I built myself lol). You can try compiling Acridotheres yourself, though.

## License

This project is licensed under [MIT license](LICENSE).

---

<p align="center">
  <a href="#start-of-content">
    <img src="https://banner.acridotheres.com/star.jpg" alt="Please star this repository :)">
  </a>
</p>
