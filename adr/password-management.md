Issue: The ERP should allow managment of employe and contractor passwords. Only each employee should be able to open their password vault, but managers should be able to add and remove passwords in their employees vaults

Decision: I'm going to use [pass](https://www.passwordstore.org/) to manage passwords

Status: pending (until I know I can use user supplied public keys to creates password stores)

Group: CLI tools

Assumptions: 
    - We can use user supplied public keys
    - I'll be able to build a desktop app that downloads the store from the erpnext server and takes care of managing GPG keys for less able users
    - I'll be able to run cli commands from a python script in a frappe instance
    - Recovering from database filesystem desyncs shouldn't prove too difficult

Constraints: We'll need to relie on the filesytem to store state as opposed to the database with regards to password management

Positions: Only the person that is meant to use the password should have access to the password. This means that managers who can add passwords and the server that stores the password should never be able to read to contents of the password vault. There should be no "master key". However propigating password changes should be simple, straight forward and instant.

Argument:
    - This tool supplies a simple cli interface
    - It's commonly using in the linux community
    - it stores the passwords in plain text
    - vaults can be stored in a git repo allowing "lookback" features incase an important password was deleted
    - If the vaults are downloaded users can use standard tools to interact with the vault
    - I can trust that the application is securly built

Implications: Since the server won't have access to the private keys, users would need a phone or desktop application to store the keys and read the passwords (I can make this optional by having the server create the GPG key pair). The python executor will need write access on certain dirctories. 

Related decisions: None

Related requirements: None

Related artifacts: None

Related principles: CLI tools and the UNIX philosophy are an effective aids in building good processes

Notes: pass vaults can be managed with other non cli tools like
    - [Android-Password-Store](https://github.com/android-password-store/Android-Password-Store#readme)
    - [dmenu - password-store](https://git.zx2c4.com/password-store/tree/contrib/dmenu)
    - [OS X integration](https://git.zx2c4.com/password-store/tree/contrib/pass.applescript)
    - [Firefox plugin](https://github.com/passff/passff#readme)
    - [Chrome plugin](https://github.com/browserpass/browserpass-extension)
    - [Windows](https://github.com/mbos/Pass4Win#readme)
    - [IOS](https://mssun.github.io/passforios/)
