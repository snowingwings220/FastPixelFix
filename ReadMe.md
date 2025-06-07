### What is this? ###

(A remake of https://github.com/Corecii/Transparent-Pixel-Fix)

As an optimizaton, a lot of image editors export fully transparent pixels as black, however some software (in my case roblox studio) does not account for fully transparent pixels in the bilinear scaling, resulting in strange black edges

### How does this program fix this? ###

It averages the color of the image and sets the background to that.

### Why was this made? ###

The original script took forever on 4k images (it really annoyed me), so this project attempts to lower the processing time by using parralel computing (& a compiled language)
I hope this is useful to anyone who was upset with the processing times of the previous solution

PS: This is not meant to be hate towards to original creator, their tool was incredibly useful to me in the past, just thought I could improve it!