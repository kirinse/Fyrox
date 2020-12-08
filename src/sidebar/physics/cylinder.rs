use crate::{
    gui::{BuildContext, Ui, UiMessage, UiNode},
    physics::Collider,
    scene::{SceneCommand, SetCylinderHalfHeightCommand, SetCylinderRadiusCommand},
    sidebar::{make_f32_input_field, make_text_mark, COLUMN_WIDTH, ROW_HEIGHT},
    Message,
};
use rg3d::{
    core::pool::Handle,
    gui::{
        grid::{Column, GridBuilder, Row},
        message::{MessageDirection, NumericUpDownMessage, UiMessageData},
        widget::WidgetBuilder,
    },
    scene::physics::CylinderDesc,
};
use std::sync::mpsc::Sender;

pub struct CylinderSection {
    pub section: Handle<UiNode>,
    half_height: Handle<UiNode>,
    radius: Handle<UiNode>,
    sender: Sender<Message>,
}

impl CylinderSection {
    pub fn new(ctx: &mut BuildContext, sender: Sender<Message>) -> Self {
        let half_height;
        let radius;
        let section = GridBuilder::new(
            WidgetBuilder::new()
                .with_child(make_text_mark(ctx, "Half Height", 0))
                .with_child({
                    half_height = make_f32_input_field(ctx, 0);
                    half_height
                })
                .with_child(make_text_mark(ctx, "Radius", 1))
                .with_child({
                    radius = make_f32_input_field(ctx, 1);
                    radius
                }),
        )
        .add_column(Column::strict(COLUMN_WIDTH))
        .add_column(Column::stretch())
        .add_row(Row::strict(ROW_HEIGHT))
        .add_row(Row::strict(ROW_HEIGHT))
        .build(ctx);

        Self {
            section,
            sender,
            half_height,
            radius,
        }
    }

    pub fn sync_to_model(&mut self, cylinder: &CylinderDesc, ui: &mut Ui) {
        ui.send_message(NumericUpDownMessage::value(
            self.half_height,
            MessageDirection::ToWidget,
            cylinder.half_height,
        ));

        ui.send_message(NumericUpDownMessage::value(
            self.radius,
            MessageDirection::ToWidget,
            cylinder.radius,
        ));
    }

    pub fn handle_message(&mut self, message: &UiMessage, handle: Handle<Collider>) {
        if let UiMessageData::NumericUpDown(msg) = message.data() {
            if let &NumericUpDownMessage::Value(value) = msg {
                if message.direction() == MessageDirection::FromWidget {
                    if message.destination() == self.half_height {
                        self.sender
                            .send(Message::DoSceneCommand(
                                SceneCommand::SetCylinderHalfHeight(
                                    SetCylinderHalfHeightCommand::new(handle, value),
                                ),
                            ))
                            .unwrap();
                    } else if message.destination() == self.radius {
                        self.sender
                            .send(Message::DoSceneCommand(SceneCommand::SetCylinderRadius(
                                SetCylinderRadiusCommand::new(handle, value),
                            )))
                            .unwrap();
                    }
                }
            }
        }
    }
}
