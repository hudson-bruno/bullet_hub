use std::sync::mpsc::{channel, Receiver, Sender};

use godot::{engine::CsgBox3D, prelude::*};
use spacetimedb_sdk::{
    disconnect,
    identity::{load_credentials, once_on_connect, save_credentials, Credentials},
    on_disconnect, subscribe,
    table::{TableType, TableWithPrimaryKey},
    Address,
};

use crate::module_bindings::*;

const CREDS_DIR: &str = ".spacetime_chat";

#[derive(Debug)]
pub enum SpaceTimeEvents<T> {
    Insert(T),
    Update(T, T),
    Delete(T),
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct BulletHubServer {
    #[base]
    base: Base<Node>,

    #[export]
    server: GString,
    #[export]
    database: GString,

    tx: Sender<SpaceTimeEvents<DynamicEntity>>,
    rx: Receiver<SpaceTimeEvents<DynamicEntity>>,
}

#[godot_api]
impl INode for BulletHubServer {
    fn init(base: Base<Node>) -> Self {
        let (tx, rx) = channel::<SpaceTimeEvents<DynamicEntity>>();

        Self {
            base,
            server: "http://localhost:3000".into(),
            database: "database".into(),
            tx,
            rx,
        }
    }

    fn ready(&mut self) {
        self.register_callbacks();

        if let Err(e) = self.connect_to_db() {
            godot_warn!("connection error: {e}");
            return;
        }

        self.subscribe_to_tables();
    }

    fn process(&mut self, _delta: f64) {
        if let Ok(event) = self.rx.try_recv() {
            godot_print!("event: {:?}", event);
            match event {
                SpaceTimeEvents::Insert(entity) => {
                    let mut node_entity = CsgBox3D::new_alloc();
                    node_entity.set_name(entity.rigid_body_id.to_string().into_godot());

                    self.base.add_child(node_entity.upcast::<Node>());
                }
                SpaceTimeEvents::Update(_, entity) => {
                    if let Some(child) = self.base.get_child(entity.rigid_body_id as i32) {
                        child.cast::<CsgBox3D>().set_global_position(Vector3::new(
                            entity.position.x,
                            entity.position.y,
                            0.0,
                        ));
                    }
                }
                SpaceTimeEvents::Delete(entity) => {
                    if let Some(mut child) = self.base.get_child(entity.rigid_body_id as i32) {
                        child.queue_free();
                    }
                }
            }
        }
    }

    fn exit_tree(&mut self) {
        disconnect();
    }
}

#[godot_api]
impl BulletHubServer {
    fn register_callbacks(&mut self) {
        once_on_connect(on_connected);

        let tx_insert = self.tx.clone();
        DynamicEntity::on_insert(move |new: &DynamicEntity, _: Option<&ReducerEvent>| {
            tx_insert
                .send(SpaceTimeEvents::Insert(new.clone()))
                .expect("Error sending spacetime insert message");
        });

        let tx_update = self.tx.clone();
        DynamicEntity::on_update(
            move |old_entity: &DynamicEntity,
                  new_entity: &DynamicEntity,
                  _: Option<&ReducerEvent>| {
                tx_update
                    .send(SpaceTimeEvents::Update(
                        old_entity.clone(),
                        new_entity.clone(),
                    ))
                    .expect("Error sending spacetime updated message");
            },
        );

        let tx_delete = self.tx.clone();
        DynamicEntity::on_delete(move |deleted: &DynamicEntity, _: Option<&ReducerEvent>| {
            tx_delete
                .send(SpaceTimeEvents::Delete(deleted.clone()))
                .expect("Error sending spacetime deleted message");
        });

        on_disconnect(on_disconnected);
    }

    fn connect_to_db(&mut self) -> Result<(), String> {
        let credentials =
            load_credentials(CREDS_DIR).map_err(|e| format!("Credentials not found: {:?}", e))?;

        if let None = credentials {
            godot_warn!("Credentials not found");
        }

        connect(
            &self.server.to_string(),
            &self.database.to_string(),
            credentials,
        )
        .map_err(|e| format!("Failed to connect: {:?}", e))?;

        Ok(())
    }

    fn subscribe_to_tables(&mut self) {
        subscribe(&["SELECT * FROM DynamicEntity;"]).unwrap();
    }
}
fn on_connected(creds: &Credentials, _client_address: Address) {
    if let Err(e) = save_credentials(CREDS_DIR, creds) {
        godot_warn!("Failed to save credentials: {:?}", e);
    }

    godot_print!("Connected to Server!")
}

fn on_disconnected() {
    godot_warn!("Disconnected!");
}
