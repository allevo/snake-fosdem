import * as wasm from "handly-made-snake"

wasm.set_panic_hook()

chooseGame()
    .then(levelName => startGame(levelName))

function chooseGame() {
    const levels = wasm.levels()
    const div = document.querySelector('.choose-game')

    for (let levelName of Object.keys(levels)) {
        const game = wasm.create_game(levelName)
        const wallsObject = calculateWallsObject(game)

        div.innerHTML += '<div class="example ' + levelName + '" data-level-name="' + levelName + '"><div>' + levelName + '</div><div class="board"></div></div>';

        const el = document.querySelector('.choose-game .example.' + levelName + ' .board')
        draw(game, el, wallsObject)
    }

    return new Promise((res) => {
        div.addEventListener('click', function (ev) {
            div.style.display = 'none'
            let parent = ev.target.parentNode
            let levelName = parent.getAttribute('data-level-name');
            res(levelName)
        })
    })
    
}

let interval
let direction = 0
function startGame(level) {
    const game = wasm.create_game(level)
    const wallsObject = calculateWallsObject(game)

    const gameEl = document.getElementById('game')
    gameEl.style.display = "flex"
    const gameBoardEl = document.getElementById('game-board')
    gameBoardEl.style.display = "inherit"
    const dieReasonEl = document.getElementById('die-reason')
    const scoreEl = document.getElementById('score')

    const snapshot = game.snapshot()

    let duration = snapshot.period_duration_ms()
    interval = setInterval(run, duration)

    draw(game, gameBoardEl, wallsObject)
    function run () {
        console.log('run!')

        let snapshot = game.tick(direction)

        draw(game, gameBoardEl, wallsObject)
        
        let score = snapshot.score();
        scoreEl.textContent = `score: ${score}`

        let reason = snapshot.die_reason();
        if (reason) {
            dieReasonEl.textContent += reason;
            console.log('ENDED!')
            clearInterval(interval)
            return
        }

        let new_duration = snapshot.period_duration_ms()
        if (duration !== new_duration) {
            duration = new_duration
            console.log('clear & setInterval!', duration)
            clearInterval(interval)
            interval = setInterval(run, duration)
        }
    }
}

function calculateWallsObject(game) {
    const walls = game.walls()
    const wallsObject = {}
    for (let i = 0; i < walls.length; i += 2) {
        let x = walls[i]
        let y = walls[i + 1]
        wallsObject[`${x}-${y}`] = true
    }

    return wallsObject
}

function draw(game, el, wallsObject) {
    const dim = game.dim()
    const snapshot = game.snapshot()
    const width = dim[0]
    const height = dim[1]

    let snake = snapshot.snake()
    let food = snapshot.food()

    let head_x = snake[0]
    let head_y = snake[1]

    const body = []
    for (let i = 2; i < snake.length; i += 2) {
        let x = snake[i]
        let y = snake[i + 1]
        body.push(`${x}-${y}`)
    }

    let content = '';

    for (let y = 0; y < height; y++) {
        let line = "";
        for (let x = 0; x < width; x ++) {
            if (head_x === x && head_y === y) {
                line += '🅾️'
            } else if (body.includes(`${x}-${y}`)) {
                line += '❎'
            } else if (food[0] === x && food[1] === y) {
                line += '🍒'
            } else if (wallsObject[`${x}-${y}`]) {
                line += '⬛'
            } else {
                line += '⬜'
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