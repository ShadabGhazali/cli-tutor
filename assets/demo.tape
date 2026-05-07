Output assets/demo.gif

Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 700
Set Theme "TokyoNight"
Set Padding 20
Set Framerate 24
Set PlaybackSpeed 0.80
Set TypingSpeed 80ms

# Launch cli-tutor
Type "cli-tutor"
Enter
Sleep 2s

# Navigate down to grep (5th module)
Down
Sleep 300ms
Down
Sleep 300ms
Down
Sleep 300ms
Down
Sleep 800ms

# Scroll the intro
Ctrl+D
Sleep 600ms
Ctrl+D
Sleep 600ms
Ctrl+U
Sleep 600ms

# Switch to Examples view
Tab
Sleep 1.2s

# Switch to Exercise view
Tab
Sleep 1.5s

# Type a wrong answer first
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

# Show the progress overlay
Sleep 400ms
Ctrl+C
Sleep 500ms
