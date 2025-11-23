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
1. Somehow notify the user when the timer expires. Play a sound or flash the terminal window.
1. Probably refactor main.rs to get the app code out of there.

Ideas:
- Create a tool for drawing big letter asci art chars
  - Could use ai? Cooler to use something more old school
