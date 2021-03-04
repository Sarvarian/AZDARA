class_name CallBackObject
extends Object


var rid : RID


func monitor_callback(
	area_body_state : int,
	object_rid : RID,
	object_instance_id : int,
	object_shape_index : int,
	area_shape_index : int
	) -> void:
	
	pass

func force_integration_callback(
	state : PhysicsDirectBodyState,
	userdata
	) -> void:
	
	var x = PhysicsBody
	
	pass








