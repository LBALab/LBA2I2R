extern crate image;

use self::image::{DynamicImage, Rgba, GenericImage};
use std::fs::File;
use std::io::{Error, ErrorKind, Write, Seek, SeekFrom, Read};
use crate::fileheader::Fileheader;

// Define palette
const PALETTE: [[u8; 3]; 256] = [
    [0, 0, 0],       // Color 0
    [116, 68, 68],   // Color 1
    [212, 124, 124], // Color 2
    [228, 164, 168], // Color 3
    [0, 0, 112],     // Color 4
    [0, 0, 168],     // Color 5
    [0, 0, 212],     // Color 6
    [84, 252, 252],  // Color 7
    [220, 0, 124],   // Color 8
    [220, 0, 124],   // Color 9
    [220, 0, 124],   // Color 10
    [220, 0, 124],   // Color 11
    [220, 0, 124],   // Color 12
    [220, 0, 124],   // Color 13
    [220, 0, 124],   // Color 14
    [252, 252, 252], // Color 15
    [4, 20, 32],     // Color 16
    [4, 24, 40],     // Color 17
    [8, 28, 48],     // Color 18
    [12, 36, 56],    // Color 19
    [16, 40, 64],    // Color 20
    [20, 48, 72],    // Color 21
    [24, 52, 80],    // Color 22
    [32, 60, 88],    // Color 23
    [36, 64, 96],    // Color 24
    [40, 68, 104],   // Color 25
    [48, 76, 116],   // Color 26
    [56, 84, 124],   // Color 27
    [60, 88, 128],   // Color 28
    [68, 92, 136],   // Color 29
    [72, 96, 140],   // Color 30
    [76, 104, 144],  // Color 31
    [84, 108, 152],  // Color 32
    [88, 112, 156],  // Color 33
    [92, 120, 164],  // Color 34
    [100, 124, 172], // Color 35
    [104, 132, 176], // Color 36
    [108, 136, 184], // Color 37
    [112, 144, 192], // Color 38
    [120, 152, 200], // Color 39
    [128, 164, 204], // Color 40
    [136, 176, 212], // Color 41
    [148, 192, 220], // Color 42
    [160, 200, 224], // Color 43
    [168, 216, 232], // Color 44
    [180, 224, 236], // Color 45
    [192, 236, 244], // Color 46
    [204, 248, 252], // Color 47
    [16, 20, 20],    // Color 48
    [24, 28, 28],    // Color 49
    [36, 40, 40],    // Color 50
    [44, 52, 52],    // Color 51
    [56, 64, 64],    // Color 52
    [64, 76, 76],    // Color 53
    [76, 84, 84],    // Color 54
    [88, 96, 96],    // Color 55
    [100, 108, 108], // Color 56
    [112, 120, 120], // Color 57
    [132, 140, 140], // Color 58
    [156, 164, 164], // Color 59
    [180, 184, 184], // Color 60
    [200, 208, 208], // Color 61
    [224, 228, 228], // Color 62
    [252, 252, 252], // Color 63
    [0, 0, 44],      // Color 64
    [0, 0, 60],      // Color 65
    [0, 0, 76],      // Color 66
    [0, 0, 92],      // Color 67
    [0, 0, 108],     // Color 68
    [0, 0, 124],     // Color 69
    [4, 4, 136],     // Color 70
    [12, 16, 148],   // Color 71
    [24, 24, 164],   // Color 72
    [36, 40, 176],   // Color 73
    [52, 56, 188],   // Color 74
    [72, 76, 200],   // Color 75
    [96, 100, 212],  // Color 76
    [116, 120, 224], // Color 77
    [144, 148, 236], // Color 78
    [172, 176, 252], // Color 79
    [4, 4, 40],      // Color 80
    [8, 8, 60],      // Color 81
    [16, 20, 80],    // Color 82
    [20, 28, 104],   // Color 83
    [28, 40, 124],   // Color 84
    [40, 56, 144],   // Color 85
    [48, 68, 168],   // Color 86
    [60, 84, 188],   // Color 87
    [72, 104, 208],  // Color 88
    [84, 124, 232],  // Color 89
    [100, 144, 252], // Color 90
    [120, 164, 248], // Color 91
    [144, 184, 248], // Color 92
    [168, 204, 248], // Color 93
    [188, 220, 248], // Color 94
    [212, 232, 248], // Color 95
    [16, 28, 36],    // Color 96
    [20, 40, 48],    // Color 97
    [28, 52, 64],    // Color 98
    [36, 64, 80],    // Color 99
    [40, 76, 96],    // Color 100
    [44, 88, 112],   // Color 101
    [52, 100, 128],  // Color 102
    [60, 116, 152],  // Color 103
    [68, 136, 176],  // Color 104
    [72, 152, 200],  // Color 105
    [80, 172, 224],  // Color 106
    [84, 184, 244],  // Color 107
    [108, 196, 244], // Color 108
    [136, 208, 244], // Color 109
    [160, 216, 244], // Color 110
    [188, 228, 244], // Color 111
    [20, 32, 20],    // Color 112
    [24, 40, 24],    // Color 113
    [32, 48, 32],    // Color 114
    [36, 56, 40],    // Color 115
    [44, 68, 48],    // Color 116
    [48, 76, 56],    // Color 117
    [52, 84, 68],    // Color 118
    [60, 92, 76],    // Color 119
    [64, 100, 88],   // Color 120
    [72, 112, 100],  // Color 121
    [84, 128, 116],  // Color 122
    [100, 148, 136], // Color 123
    [120, 168, 152], // Color 124
    [136, 188, 172], // Color 125
    [152, 208, 192], // Color 126
    [172, 228, 212], // Color 127
    [8, 28, 4],      // Color 128
    [12, 32, 4],     // Color 129
    [16, 40, 8],     // Color 130
    [20, 48, 12],    // Color 131
    [24, 56, 16],    // Color 132
    [28, 64, 20],    // Color 133
    [32, 72, 24],    // Color 134
    [40, 84, 32],    // Color 135
    [52, 100, 44],   // Color 136
    [68, 120, 60],   // Color 137
    [84, 140, 76],   // Color 138
    [104, 160, 96],  // Color 139
    [124, 180, 116], // Color 140
    [144, 200, 136], // Color 141
    [168, 224, 160], // Color 142
    [192, 244, 188], // Color 143
    [24, 24, 0],     // Color 144
    [32, 32, 0],     // Color 145
    [40, 40, 0],     // Color 146
    [44, 48, 0],     // Color 147
    [52, 56, 4],     // Color 148
    [60, 64, 4],     // Color 149
    [72, 76, 8],     // Color 150
    [80, 88, 16],    // Color 151
    [92, 104, 24],   // Color 152
    [100, 116, 36],  // Color 153
    [112, 132, 52],  // Color 154
    [120, 144, 56],  // Color 155
    [136, 164, 64],  // Color 156
    [152, 184, 72],  // Color 157
    [168, 204, 80],  // Color 158
    [184, 224, 88],  // Color 159
    [24, 16, 0],     // Color 160
    [32, 20, 0],     // Color 161
    [40, 28, 0],     // Color 162
    [48, 40, 0],     // Color 163
    [60, 48, 0],     // Color 164
    [68, 56, 4],     // Color 165
    [80, 68, 4],     // Color 166
    [92, 80, 8],     // Color 167
    [104, 92, 12],   // Color 168
    [116, 104, 16],  // Color 169
    [128, 120, 24],  // Color 170
    [152, 144, 44],  // Color 171
    [176, 172, 68],  // Color 172
    [200, 200, 96],  // Color 173
    [224, 224, 128], // Color 174
    [252, 252, 168], // Color 175
    [28, 24, 16],    // Color 176
    [36, 32, 20],    // Color 177
    [44, 40, 24],    // Color 178
    [52, 48, 32],    // Color 179
    [60, 56, 36],    // Color 180
    [72, 68, 44],    // Color 181
    [80, 76, 52],    // Color 182
    [92, 88, 64],    // Color 183
    [104, 100, 72],  // Color 184
    [116, 112, 80],  // Color 185
    [128, 124, 88],  // Color 186
    [140, 136, 100], // Color 187
    [160, 156, 116], // Color 188
    [180, 176, 136], // Color 189
    [200, 196, 160], // Color 190
    [224, 220, 184], // Color 191
    [36, 16, 4],     // Color 192
    [48, 20, 12],    // Color 193
    [60, 36, 12],    // Color 194
    [72, 48, 20],    // Color 195
    [84, 60, 24],    // Color 196
    [100, 72, 24],   // Color 197
    [116, 84, 28],   // Color 198
    [132, 100, 32],  // Color 199
    [148, 116, 32],  // Color 200
    [164, 132, 36],  // Color 201
    [176, 140, 56],  // Color 202
    [188, 152, 76],  // Color 203
    [200, 164, 100], // Color 204
    [212, 180, 128], // Color 205
    [224, 196, 156], // Color 206
    [236, 216, 188], // Color 207
    [24, 20, 24],    // Color 208
    [32, 28, 32],    // Color 209
    [44, 36, 40],    // Color 210
    [52, 44, 52],    // Color 211
    [64, 52, 60],    // Color 212
    [72, 60, 68],    // Color 213
    [84, 68, 80],    // Color 214
    [96, 80, 88],    // Color 215
    [108, 92, 100],  // Color 216
    [120, 104, 112], // Color 217
    [132, 116, 124], // Color 218
    [148, 128, 136], // Color 219
    [172, 152, 160], // Color 220
    [196, 176, 184], // Color 221
    [220, 204, 208], // Color 222
    [248, 232, 236], // Color 223
    [8, 8, 12],      // Color 224
    [16, 16, 24],    // Color 225
    [24, 24, 36],    // Color 226
    [36, 36, 48],    // Color 227
    [44, 44, 60],    // Color 228
    [56, 56, 72],    // Color 229
    [64, 64, 84],    // Color 230
    [76, 80, 96],    // Color 231
    [92, 96, 112],   // Color 232
    [108, 112, 128], // Color 233
    [124, 128, 144], // Color 234
    [140, 144, 160], // Color 235
    [160, 164, 176], // Color 236
    [176, 180, 192], // Color 237
    [192, 200, 212], // Color 238
    [212, 220, 232], // Color 239
    [0, 0, 0],       // Color 240
    [0, 92, 228],    // Color 241
    [0, 124, 236],   // Color 242
    [0, 164, 244],   // Color 243
    [0, 204, 252],   // Color 244
    [0, 252, 252],   // Color 245
    [252, 84, 84],   // Color 246
    [84, 252, 84],   // Color 247
    [252, 252, 84],  // Color 248
    [84, 84, 252],   // Color 249
    [252, 84, 252],  // Color 250
    [252, 252, 252], // Color 251
    [252, 252, 252], // Color 252
    [252, 252, 252], // Color 253
    [252, 252, 252], // Color 254
    [252, 252, 252], // Color 255
];


pub fn extract_raw_data(header: &Fileheader, image: &DynamicImage) -> Result<Vec<u8>, Error> {
    // Resize the image to 160x120
    let resized_image = image.resize_exact(160, 120, image::imageops::FilterType::Nearest);

    // Convert the resized image to RGBA format
    let resized_image = resized_image.into_rgba8();

    // Extract raw pixel data
    let mut raw_data = Vec::new();
    for (_, _, pixel) in resized_image.enumerate_pixels() {
        let palette_index = find_palette_index(*pixel);
        raw_data.push(palette_index as u8);
    }

    // Check if the extracted raw data size matches the expected size (19200 bytes)
    if raw_data.len() != 19200 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("Extracted raw data size mismatch: expected 19200 bytes, got {}", raw_data.len()),
        ));
    }

    Ok(raw_data)
}

fn find_palette_index(pixel: Rgba<u8>) -> usize {
    // Iterate through the palette to find the closest match for the given pixel
    // This logic depends on how you define "closest match". You can use Euclidean distance or other methods.
    // Here, we're simply finding the palette index with the closest RGB values.
    let mut min_distance = u32::MAX;
    let mut closest_index = 0;

    for (index, &palette_color) in PALETTE.iter().enumerate() {
        let r_diff = (palette_color[2] as i32) - (pixel[0] as i32);
        let g_diff = (palette_color[1] as i32) - (pixel[1] as i32);
        let b_diff = (palette_color[0] as i32) - (pixel[2] as i32);

        let distance = (r_diff * r_diff + g_diff * g_diff + b_diff * b_diff) as u32;

        if distance < min_distance {
            min_distance = distance;
            closest_index = index;
        }
    }

    closest_index
}

pub fn create_image(raw_data: &[u8]) -> DynamicImage {
    let mut image = DynamicImage::new_rgba8(160, 120);

    // Transcribe raw pixel data to image using the palette
    for (index, &palette_index) in raw_data.iter().enumerate() {
        let x = index as u32 % 160;
        let y = index as u32 / 160;
        let palette_color = PALETTE[palette_index as usize];

        let color = image::Rgba([palette_color[0], palette_color[1], palette_color[2], 255]);
        image.put_pixel(x, y, color);
    }

    image
}

pub fn write_to_file(raw_data: &[u8], mut file: File, header: &Fileheader) -> Result<(), Error> {
    // Seek to the position after the header
    file.seek(SeekFrom::Start(header.header_end))?;

    // Write the raw pixel data to the file
    file.write_all(raw_data)?;

    // Seek back to the beginning of the file
    file.seek(SeekFrom::Start(0))?;

    // Read and discard the content up to the end of the header
    let mut buffer = vec![0; header.header_end as usize];
    file.read_exact(&mut buffer)?;

    // Write back the header content
    file.seek(SeekFrom::Start(0))?;
    file.write_all(&buffer)?;

    Ok(())
}