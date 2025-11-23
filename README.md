# Tomate

What is tomate? A pomodoro timer that can sit in a terminal window. It has a minimal feature set.
This isn't meant to be a particularly fancy tool. It allows you to:
- Start a 25 minute timer, shortcut 'w' -> start work
- Start a 5 minute timer, shortcut 'b' -> start break

While a timer is running, you can:
- Pause the timer
- Resume the timer
- Reset the timer

You do this by using specified keystrokes:
- Press 'p' to pause the timer
- Press 'r' to resume the timer
- Press 'x' to reset the timer

Roadmap:
1. Rather than displaying the remaining time as a small number, format it as a 
logo or similar with big block letters.
2. When timer isn't running, Rather than displaying the time, display a message indicating
you can start the timer with w or b
3. Somehow notify the user when the timer expires. Play a sound or flash the terminal window.
4. Rather than using a single stroke of x to reset the timer, use a double stroke of x to reset the timer.
