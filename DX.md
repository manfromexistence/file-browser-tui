Currently our screens have left and right key navigation but they are without any reason so we have to put actual useful functionalities to them so they will work like this:
1. The splash screen will show.
2. When we click the right arrow from the splash screen, it will go to the file browser.
3. When we click the left arrow from the splash screen, it will go to the matrix animation.
Now from the splash screen whenever we will put the right arrow key, it will always go to only the file browser screen and when we click the left arrow key from the splash screen, it will go to the matrix animation screen. From there we will apply some logic where we will list all of the screens, right? From there please remove the current spinner screen as it was just a temporary screen and keep all the other animation screens.
The functionality is here that in the matrix animation screen carousel we will use the top and bottom arrow keys to set the message list intro and outro animation:
- Up button means we have selected this animation screen for the message list intro animation.
- Down button means we have selected the animation screen as the outro animation screen of the mesh screen.
The splash screen will be the default one. From the splash screen whenever we click on the right arrow key on the arrow click button we will always go to the file browser. When we click on the left arrow key we will go to the animations carousel where we can select what will be the intro and outro animations when we enter the message list and go outside of the message list.
Now ask me any questions if you have any clarification question so that I can answer it!!!
----------------------------------------------------------------------------------------------------------------------
In our animations there are train, spinner screen - the train is already commenout out so keep it that way and now comment the spinner screen too!
1. without the train,spinner, splash, file browser than all other screens
2. do what you think is best
3. The intro means when we put anything in the input box and press Enter we will go to the message list, right? When we go to the message list, between those transitions, show the animation. It is called an intro animation and when we get back to the screen, to the splash screen, it is called an outro animation. Now all the animation screens can be intro or outro animations. One single screen can also be both intro and outro animations. On the top right we will show a toast and a notification about this being the current intro or outro animation and when we press up or down we will show that it has been updated to be the new intro or outro animation. 
4. as I told you in 3rd clarification please show them correctly with top right toast with our theme colors
5. just check the screens and you will find it
----------------------------------------------------------------------------------------------------------------------
No, intro outro animatino is showing + From the splash screen when we click on the right arrow key, it's currently going to the file browser. Guess what? Now from the file browser, no matter the left or right arrow key, it will always go to the splash screen every time. When we click the left arrow key from the splash screen, it will go to the matrix animation carousel list. Now from that point when we click the right arrow key, it will go to the splash screen and when we click the left arrow key it will show other animation. Make sure that when we are in the carousel we will not see the all and when we are in the file browser we will not see any other animation screen at all. 

Now in our Tui please use this "Block" spinner at the spinner screen and when we hold the space key, the spinner will show up on the right of the chat input box. As the chat input box already has the border, it will place itself in the most right of the chat input box.

Currently in the chat at input bottom center we are only showing three messages on everywhere and timely animation. That is good. Make sure when we are on the file browser screen we should first show the tips and have related details about the file browser, like how, when you place the left or right arrow, it will go to the slash screen and things like that. When we are in the animation carousel we will show how clicking on the top and bottom arrow key will select the intro and outro and things like that.

Now even in the intro and outro please show the chat bottom and chat action buttons. 

Now give me brutal thruhts do we have any clippy right now or not?? And the clippy allow wanings don't count as those are not critical - silence all wanings that you find and even this wanings "warning: dx-tui@26.2.2: Embedded 113 figlet fonts (221566 bytes compressed)"
