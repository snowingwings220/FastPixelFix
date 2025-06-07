### What is this? ###

(A remake of https://github.com/Corecii/Transparent-Pixel-Fix)

As an optimization, a lot of image editors export fully transparent pixels as black, however some software (in my case Roblox Studio) does not account for fully transparent pixels in the bilinear scaling, resulting in strange black edges

### How does this program fix this? ###

It averages the color of the image and sets the background to that.

### Why was this made? ###

The original script took forever on 4k images (it really annoyed me), so this project attempts to lower the processing time by using parallel computing (& a compiled language)
I hope this is useful to anyone who was upset with the processing times of the previous solution

PS: This is not meant to be hate towards the original creator, their tool was incredibly useful to me in the past, just thought I could improve it!


### Can I use this in the shell:sendto folder? ###

Probably, untested as of now.

### How big of a file size can be ran? ###

Depends on how many logical processors your cpu has, I was able to do 2134x2134 in ~0.24 seconds, however I have 24 processors and your speeds may vary.

### How about rewriting this to use CUDA or another GPU programming framework? ###

Personally, I think this version works fine for me. 
I think i'd be a fun project, but speed increases wouldn't be worth it for 4k images (which is what I need it for mostly).