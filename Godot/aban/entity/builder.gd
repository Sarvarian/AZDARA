class_name EntityBuilder
extends Object


static func build(data : Dictionary, entity_scene : PackedScene) -> Node:
	var entity : Node = entity_scene.instance()
	if_static(entity, data)
	if_movable(entity, data)
	if_player(entity, data)
	return entity


static func if_static(entity : Entity, data : Dictionary) -> void:
	if data.has("static"):
		entity.set_position(Vector2(data["static"].x, data["static"].y) * 50)
		entity.set_static()


static func if_movable(entity : Entity, data : Dictionary) -> void:
	if data.has("movable"):
		entity.set_position(Vector2(data.movable.x, data.movable.y) * 50)


static func if_player(entity : Node, data : Dictionary) -> void:
	if data.has("player"):
		var player_input : PlayerInput = PlayerInput.new()
		player_input.player_id = data.player
		entity.add_child(player_input)
