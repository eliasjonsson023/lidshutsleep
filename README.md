# lidshutsleep
Polls the linux internals every 5 seconds to see if the laptop lid is closed. Makes system sleep if true.
This makes sure the laptop will go back to suspended state and prevents the system from running idle when it's not supposed to be active.

This simple application was the answer to my linux laptop waking up in my backpack, making it hot and depleting the battery when it was not supposed to.


1. Disable the system's automatic suspension upon closing the lid, lidshutsleep will handle that.
2. Install rust at https://www.rust-lang.org/learn/get-started
3. Run the install scripts, step1 to step3. Step 2 and 3 needs sudo to run.
4. Done. Now your laptop will automatically sleep within 5 seconds upon closing the laptop lid and return to sleep again if the system for some reason wakes up when the lid is still closed.
