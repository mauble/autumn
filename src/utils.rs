use bevy_spritesheet_animation::prelude::*;
use std::collections::HashMap;

pub fn create_directional_animations(
    library: &mut AnimationLibrary,
    name: &str,
    frame_ranges: HashMap<&str, (usize, usize)>,
    duration: AnimationDuration,
    repetitions: AnimationRepeat,
) {
    for (direction, (start_frame, end_frame)) in frame_ranges {
        let frames: Vec<usize> = (start_frame..=end_frame).collect();
        let clip = Clip::from_frames(frames).with_duration(duration);
        let clip_id = library.register_clip(clip);

        let animation = Animation::from_clip(clip_id).with_repetitions(repetitions);
        let animation_id = library.register_animation(animation);
        let _ = library.name_animation(animation_id, format!("{}_{}", name, direction));
    }
}
