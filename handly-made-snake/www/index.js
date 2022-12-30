import * as wasm from "handly-made-snake"

wasm.set_panic_hook()

chooseGame()

function chooseGame() {
    const levels = wasm.levels()
    const div = document.querySelector('.choose-game')

    for (let levelName of Object.keys(levels)) {
        const game = wasm.create_game(levelName)
        const walls = game.walls()
        const wallsObject = {}
        for (let i = 0; i < walls.length; i += 2) {
            let x = walls[i]
            let y = walls[i + 1]
            wallsObject[`${x}-${y}`] = true
        }

        div.innerHTML += '<div class="example ' + levelName + '" data-level-name="' + levelName + '"><div>' + levelName + '</div><div class="board"></div></div>';

        const el = document.querySelector('.choose-game .example.' + levelName + ' .board')
        draw(game, el)
    }

    div.addEventListener('click', function (ev) {
        div.style.display = 'none'
        let parent = ev.target.parentNode
        let levelName = parent.getAttribute('data-level-name');
        startGame(levelName)
    })
}

let interval
let direction = 0
function startGame(level) {
    const game = wasm.create_game(level)

    const pre = document.getElementById('game-pre')
    pre.style.display = "inherit"
    const dieReason = document.getElementById('die-reason')

    const dim = game.dim()
    const width = dim[0]
    const height = dim[1]
    const walls = game.walls()
    const wallsObject = {}
    for (let i = 0; i < walls.length; i += 2) {
        let x = walls[i]
        let y = walls[i + 1]
        wallsObject[`${x}-${y}`] = true
    }

    function run () {
        let snapshot = game.tick(direction)
        let snake = snapshot.snake()
        let food = snapshot.food()

        let head_x = snake[snake.length - 2]
        let head_y = snake[snake.length - 1]

        const body = []
        for (let i = 0; i < snake.length - 2; i += 2) {
            let x = snake[i]
            let y = snake[i + 1]
            body.push(`${x}-${y}`)
        }

        let content = '';

        for (let y = 0; y < height; y++) {
            let line = "";
            for (let x = 0; x < width; x ++) {
                if (head_x === x && head_y === y) {
                    line += 'h'
                    // line += '🅾️'
                } else if (body.includes(`${x}-${y}`)) {
                    line += 'b'
                    // line += '❎'
                } else if (food[0] === x && food[1] === y) {
                    line += 'f'
                    // line += '🍒'
                } else if (wallsObject[`${x}-${y}`]) {
                    line += '#'
                } else {
                    line += ' '
                }
            }
            content = line + "\n" + content;
        }

        pre.textContent = content

        let reason = snapshot.die_reason();
        if (reason) {
            dieReason.textContent += reason;
            clearInterval(interval)
        }
    }

    run()
    interval = setInterval(run, 1000)
}

function draw(game, el) {
    const dim = game.dim()
    const snapshot = game.snapshot()
    const width = dim[0]
    const height = dim[1]
    const walls = game.walls()
    const wallsObject = {}
    for (let i = 0; i < walls.length; i += 2) {
        let x = walls[i]
        let y = walls[i + 1]
        wallsObject[`${x}-${y}`] = true
    }


    let snake = snapshot.snake()
    let food = snapshot.food()

    let head_x = snake[snake.length - 2]
    let head_y = snake[snake.length - 1]

    const body = []
    for (let i = 0; i < snake.length - 2; i += 2) {
        let x = snake[i]
        let y = snake[i + 1]
        body.push(`${x}-${y}`)
    }

    let content = '';

    for (let y = 0; y < height; y++) {
        let line = "";
        for (let x = 0; x < width; x ++) {
            if (head_x === x && head_y === y) {
                line += 'h'
                // line += '🅾️'
            } else if (body.includes(`${x}-${y}`)) {
                line += 'b'
                // line += '❎'
            } else if (food[0] === x && food[1] === y) {
                line += 'f'
                // line += '🍒'
            } else if (wallsObject[`${x}-${y}`]) {
                line += '#'
            } else {
                line += ' '
            }
        }
        content = line + "\n" + content;
    }

    el.innerHTML = content
}

document.onkeydown = checkKey;

function checkKey(e) {

    e = e || window.event;

    // up arrow
    if (e.keyCode == '38') {
        direction = 0
    }
    // down arrow
    else if (e.keyCode == '40') {
        direction = 1
    }
    // left arrow
    else if (e.keyCode == '37') {
        direction = 2
    }
    // right arrow
    else if (e.keyCode == '39') {
        direction = 3
    }
}