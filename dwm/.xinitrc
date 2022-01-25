xset dpms 60 120 180
feh --bg-scale /MyLibrary/pics/backgrounds/tree.jpg

picom -f &

while true; do

    SPACER="                   "
    START=""
    END=""

    CPU=""
    MEM=""
    DATE=""
    TIME=""

    xsetroot -name "$START$CPU$SPACER$MEM$SPACER$DATE$SPACER$TIME$END"

sleep 0.2s
done &

exec dwm
