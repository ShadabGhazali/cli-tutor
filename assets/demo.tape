Output assets/demo.gif

Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 700
Set Theme "Tokyo Night"
Set Padding 20
Set Framerate 24
Set PlaybackSpeed 0.80
Set TypingSpeed 80ms

# Launch cli-tutor
Type "cli-tutor"
Enter
Sleep 2s

# Browse the module list
Down
Sleep 400ms
Down
Sleep 400ms
Down
Sleep 400ms
Up
Sleep 400ms
Up
Sleep 400ms
Up
Sleep 800ms

# Switch to Examples view
Tab
Sleep 1.2s

# Switch to Exercise view
Tab
Sleep 1.5s

# Type a wrong answer first — makes the flow feel real
Type "grep mango fruits.txt"
Sleep 600ms
Enter
Sleep 2s

# Reset and type the correct answer
Ctrl+R
Sleep 400ms
Type "grep 'mango' fruits.txt"
Sleep 600ms
Enter
Sleep 2.5s

# Navigate to next exercise
Ctrl+N
Sleep 1s

# Reveal a hint
Ctrl+T
Sleep 1.5s

# Open the progress overlay
Sleep 400ms
# Shift+P — note: VHS sends this as two keystrokes
Sleep 2s

# Quit
Ctrl+C
Sleep 500ms
