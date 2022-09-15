# lidshutsleep
Polls the linux internals every 5 seconds to see if the laptop lid is closed. Makes system sleep if true.
This makes sure the laptop will go back to sleep state and prevents the system from remaining active when it is not supposed to be active.

This simple application was the answer to my linux laptop waking up in my backpack, making it hot and depleting the battery when it was not supposed to.


1. Disable the system's automatic suspension upon closing the lid, lidshutsleep will handle that.
2. Build the rust application through running `cargo build --release`
3. Make lidshutsleep run at startup by telling your window manager to run lidshutsleep at startup. That would be in the system settings.
4. Done. Now your laptop will automatically sleep within 5 seconds upon closing the laptop lid and return to sleep again if the system for some reason wakes up when the lid is still closed.
