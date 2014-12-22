learning_rust
=============

I learn by fiddling and experimenting. This repo houses my fiddles and experiments in Rust.

Structure
---------

As my fiddles begin prior to 1.0, my experiments will be branched per nightly date.

Notes
-----

Export dynamic library variable:
* Because I don't want to install the OS X package to root (security!), I downloaded the nightly binary. Installation to my ~/.local went smoothly, with one hitch: I need to export the dynamic library. Given security concerns, I feel very uncomfortable setting it in my .bash_profile, let alone in my startup scripts (message boards said this can be used for code injections). As such, I'll just export it every time I play with rustc.
* `export DYLD_LIBRARY_PATH="/Users/deaddork/.local/lib/rustlib/x86_64-apple-darwin/lib:$DYLD_LIBRARY_PATH"
* YMMV, but this does work for me.
* FYI as soon as you close the terminal (tab or window), the export value will go away. Only use that particular window for Rust development, and close it as soon as you're done.

