extends Tree


const t_add : Texture = preload("res://aban/game_select/new_game_card/create_story_game_panel/set_players_panel/add_64.tres")


var root : TreeItem = create_item()


func _ready() -> void:
	root.set_text(0, "Add New Player")
	root.add_button(0, t_add)
	root.set_selectable(0, false)


func _on_Tree_button_pressed(item : TreeItem, _column : int, _id : int) -> void:
	if item == root:
		print("addadd")
		pass
