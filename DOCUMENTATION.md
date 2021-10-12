This is the documentation of the rust package *grist*. This is a command-line application and is intended to be used as such

### Basic Usage
`grist`

This adds all files in your current folder to the commit queue, commits them with the default message "From my PC" and then pushes them to the remote origin specified for the folder. To change the commit message check down below[#commit messages]

### To access help
`grist -h`

### To access version
`grist -v`

This application also signs your commits by default. If you have not added your signing key to your github account, I'd urge you to do so now. It is very trivial and you can learn how to do it here[https://docs.github.com/en/authentication/managing-commit-signature-verification/adding-a-new-gpg-key-to-your-github-account].

As of now, there is no way to make signing commits optional and I have no plans of doing so in the near future. It should be easy enough to modify source code to do it anyway, should anyone want to attempt it. 

### Commit messages
`grist -m "your message here"`
