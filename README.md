# lsproj

Efficiently find all projects in a subdirectory.

```bash
$ lsproj ~/workspace
/okhumane-yule-log
/CC-Tweaked
/lsproj
/wordie
/mvim
/muse/admin-app/muse-admin-dashboard
/muse/admin-app/muse-admin-api
/muse/muse-ws
/muse/muse-api
/muse/muse-app
```

It's intended to be used for project switching/selection tools. For example, the
output may be passed to fzf, then onto a tmux script to open a session for the
project.

## Install

```bash
$ cargo install --path .
```

## Example Use Case

A script that could be used to quickly switch to a tmux session for the selected
project.
```bash
#!/bin/bash
function tsn() {
	read proj

	# normalize session name
	session_name=${proj//\//-}
	session_name=${session_name// /-}
	session_name=${session_name#-}

	echo "path: ${1}${proj}"
	echo "session_name: $session_name"

	# if tmux session exists
	#   attach
	# else
	#   create tmux session
	#   cd to project path
}

lsproj ~/workspace | fzf | tsn ~/workspace
```
