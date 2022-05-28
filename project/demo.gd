extends Timer

var progress = 0
const keys = [
	"SUPER",
	"K",
	"i",
	"t",
	"notarealkey",
	"ENTER",
	"C",
	"A",
	"T",
	";",
	"ENTER",
	"A",
	"A",
	"A",
	"A",
	".",
	"/",
	"F5",
]


func _ready():
	pass


func _on_Timer_timeout():
	get_parent().press(keys[progress])
	progress += 1
	if progress == len(keys):
		stop()
