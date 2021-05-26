extends CanvasLayer


const save_directory : String = "user://saves/"
const save_extension : String = ".azsave"
const save_card := preload("res://aban/game_select/save_card/save_card.tscn")
const new_game_card := preload("res://aban/game_select/new_game_card/new_game_card.tscn")


var saves : PoolStringArray = []
var directory : Directory = Directory.new()


onready var container : GridContainer = $Margin/SavesContainer


func _ready() -> void:
	var err : int = 0
	
	if not directory.dir_exists(save_directory):
		err = directory.make_dir_recursive(save_directory)
		if err:
			var msg := "Creating Save Directory Failed With Godot Error Code: " + String(err)
			Log.error(msg)
			push_error(msg)
	
	err = directory.open(save_directory)
	if err:
		var msg = "Opening Save Directory Failed With Godot Error Code: " + String(err)
		Log.error(msg)
		push_error(msg)
	
	err = directory.list_dir_begin(true, false)
	if err:
		var msg = "\"directory.list_dir_begin(true, false)\" for Save Directory Failed with Godot Error Code: " + String(err)
		Log.error(msg)
		push_error(msg)
	
	var save_name := directory.get_next()
	while save_name != "":
		add_to_saves(save_name)
		save_name = directory.get_next()
	
	directory.list_dir_end()
	
	make_save_cards()


func add_to_saves(save_name : String) -> void:
	if directory.current_is_dir():
		return
	if save_name.ends_with(save_extension):
		save_name.erase(save_name.length() - save_extension.length(), save_extension.length())
		saves.push_back(save_name)


func make_save_cards() -> void:
	for save_name in saves:
		var card := save_card.instance()
		card.set_name(save_name)
		container.add_child(card)


