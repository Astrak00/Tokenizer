# AulaGlobal Tokenizer

This program was inspired by the removal of the ability of obtaining the "mobiletoken" from the web-interface of AulaGlobal. It was first created as an alternative to [AGDownloader](https://github.com/Astrak00/AGDownloader)

## Usage
To use this program, go to the release section and download the appropriate binary.
To run this program, you will need to have installed:
- Firefox
- [geckodriver](https://github.com/mozilla/geckodriver) (If you have cargo, it will install it the first time you run it)

## Program Flags:
`-c / --cookie` This prevents the program from needing to launch a firefox session and skips the login. For more information on how to obtain this cookie, check [here](#)
`-t / --token_only` This limits the scope of the program to only obtain the token used in [AGDownloader](https://github.com/Astrak00/AGDownloader). 
`-s / --save_dir` Specifies where the folder where the contents will be saved.

## Experimental:
Please note the web-scrapper is not functional 100%. It leaves some files without downloading. Having said that, its fast in what it can download. If you want a better CLI and all the files, just use [AGDownloader](https://github.com/Astrak00/AGDownloader), which will have the tokenizer included at some point.

## Contributions:
If you have any contributions, please, create a pull-request and I'll review it. This has been a side project to be able to still develop the main downloader, so it might get outsated soon.
