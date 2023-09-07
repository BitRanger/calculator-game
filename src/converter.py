'''
this is a python script that converts png images to 
plain text rgb565 arrays that can be used in
numworks eadk rust apps 
'''

from PIL import Image
import numpy as np

def remove_alpha_channel(image_path):
    png = Image.open(image_path)
    png.load() # required for png.split()

    background = Image.new("RGB", png.size, (255, 255, 255))
    background.paste(png, mask=png.split()[3]) # 3 is the alpha channel
    background.save('output.png', 'PNG')

def rgb_hex565(red, green, blue):
    # take in the red, green and blue values (0-255) as 8 bit values and then combine
    # and shift them to make them a 16 bit hex value in 565 format. 
    return ("0x%0.4X" % ((int(red / 255 * 31) << 11) | (int(green / 255 * 63) << 5) | (int(blue / 255 * 31))))


def convert_png_eadk(image_path):
    image = Image.open(image_path)
    image_array = np.array(image)
    output_array = []
    output_text = "&[ "

    for x in range(0,image_array.shape[0]):
        for y in range(0,image_array.shape[1]):
            rgb_color = image_array[x,y,:]
            hex_color = rgb_hex565(rgb_color[0],rgb_color[1],rgb_color[2])
            color_text = "eadk::Color{{ rgb565: {hex} }}, "
            color_text = color_text.format(hex = hex_color)
            output_text = output_text + color_text
    '''
        &[eadk::Color{rgb565: <text> }, 
    '''
    output_text = output_text + "]"
    print(output_text)

remove_alpha_channel("res/images/white_flappybird.png") 
convert_png_eadk("output.png")