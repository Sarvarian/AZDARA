extends Tree


func _ready() -> void:
	var _root := create_item()
	hide_root = true
	for scenario in PackageManager.scenarios:
		var name := scenario as String
		if name is String:
			var item := create_item()
			item.set_text(0, name)
