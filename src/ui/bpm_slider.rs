use bevy::{
    input_focus::{
        tab_navigation::{TabIndex},
    },
    picking::hover::Hovered,
    prelude::*,
    ui_widgets::{
        CoreSliderDragState, Slider, SliderRange, SliderThumb,
        SliderValue, TrackClick
    },
};

const SLIDER_TRACK: Color = Color::srgb(0.05, 0.05, 0.05);
const SLIDER_THUMB: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub struct ValueLabel(pub Entity);

#[derive(Component)]
pub struct HorizontalSlider;

#[derive(Component)]
pub struct SliderThumbnail;

#[derive(Component)]
pub struct VerticalSlider;

pub fn horizontal_slider() -> impl Bundle {
    (
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Stretch,
            column_gap: px(4),
            height: px(12),
            width: px(200),
            ..default()
        },
        HorizontalSlider,
        Hovered::default(),
        Slider {
            track_click: TrackClick::Snap,
        },
        SliderValue(120.0),
        SliderRange::new(30.0, 300.0),
        TabIndex(0),
        Children::spawn((
            Spawn((
                Node {
                    height: px(6),
                    border_radius: BorderRadius::all(px(3)),
                    ..default()
                },
                BackgroundColor(SLIDER_TRACK),
            )),
            Spawn((
                Node {
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    left: px(0),
                    right: px(12),
                    top: px(0),
                    bottom: px(0),
                    ..default()
                },
                children![(
                    SliderThumbnail,
                    SliderThumb,
                    Node {
                        display: Display::Flex,
                        width: px(12),
                        height: px(12),
                        position_type: PositionType::Absolute,
                        left: percent(0),
                        border_radius: BorderRadius::MAX,
                        ..default()
                    },
                    BackgroundColor(SLIDER_THUMB),
                )],
            )),
        )),
    )
}

pub(crate) fn update_slider_visuals(
    sliders: Query<
        (
            Entity,
            &SliderValue,
            &SliderRange,
            &Hovered,
            &CoreSliderDragState,
            Has<VerticalSlider>,
        ),
        (
            Or<(
                Changed<SliderValue>,
                Changed<Hovered>,
                Changed<CoreSliderDragState>,
            )>,
            With<HorizontalSlider>,
        ),
    >,
    children: Query<&Children>,
    mut thumbs: Query<(&mut Node, &mut BackgroundColor, Has<SliderThumbnail>), Without<HorizontalSlider>>,
) {
    for (slider_ent, value, range, hovered, drag_state, is_vertical) in sliders.iter() {
        for child in children.iter_descendants(slider_ent) {
            if let Ok((mut thumb_node, mut thumb_bg, is_thumb)) = thumbs.get_mut(child)
                && is_thumb
            {
                let position = range.thumb_position(value.0) * 100.0;
                if is_vertical {
                    thumb_node.bottom = percent(position);
                } else {
                    thumb_node.left = percent(position);
                }

                let is_active = hovered.0 | drag_state.dragging;
                thumb_bg.0 = if is_active {
                    SLIDER_THUMB.lighter(0.3)
                } else {
                    SLIDER_THUMB
                };
            }
        }
    }
}

pub(crate) fn update_value_labels(
    sliders: Query<(&SliderValue, &ValueLabel), (Changed<SliderValue>, With<HorizontalSlider>)>,
    mut texts: Query<&mut Text>,
) {
    for (value, label) in sliders.iter() {
        if let Ok(mut text) = texts.get_mut(label.0) {
            **text = format!("{:.0} BPM", value.0);
        }
    }
}
