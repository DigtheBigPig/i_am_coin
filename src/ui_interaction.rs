use bevy::prelude::*;

pub struct UiInteractionPlugin;  

impl Plugin for UiInteractionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(jump_indicator_system)
            ;
    }
}

pub fn jump_indicator_system(
    mut interaction_query: Query<&mut Style,With<crate::ui::JumpIndicator>,>,
    jump_strength: Res<crate::player::JumpStrength>,
) {
    let jump_indicator = 100.0*jump_strength.0/crate::player::MAX_JUMP_TIME_LENGTH;
    for mut style in &mut interaction_query {
        style.size = Size::new(Val::Px(20.0), Val::Px(jump_indicator));
    }
}