#!/bin/sh

lessc less/main.less static/main.css
echo '~~~ lessc: done ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~'
CUR_WID=$(xdotool getwindowfocus)
XWID=$(xdotool search --name 'BasGit - Chromium')
xdotool windowactivate $XWID
xdotool key 'ctrl+r'
xdotool windowactivate $CUR_WID
