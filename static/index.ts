import init, { Board, Direction, GameStatus } from 'snake-game'
import { random } from './utils/random'

const CELL_SIZE = 50
const BOARD_WIDTH = 16

const start = async () => {
    const wasm = await init()

    const spawnIndex = random(BOARD_WIDTH * BOARD_WIDTH)
    const board = Board.new(BOARD_WIDTH, spawnIndex)
    const boardWidth = board.width()

    const playBtn = document.getElementById('playBtn')
    const gameStatus = document.getElementById('gameStatus')
    const gamePoints = document.getElementById('gamePoints')
    const canvas = <HTMLCanvasElement>document.getElementById('snakeCanvas')
    canvas.width = canvas.height = boardWidth * CELL_SIZE
    const context = canvas.getContext('2d')

    playBtn.addEventListener('click', () => {
        if (board.game_status() === undefined) {
            playBtn.textContent = 'O\'ynalmoqda'
            board.start_game()
            play()
        } else {
            location.reload()
        }
    })
    
    document.addEventListener('keydown', (e) => {
        switch (e.code) {
            case 'ArrowUp':
            case 'KeyW':
                board.change_snake_dir(Direction.Up)
            break
            case 'ArrowDown':
            case 'KeyS':
                board.change_snake_dir(Direction.Down)
            break
            case 'ArrowRight':
            case 'KeyD':
                board.change_snake_dir(Direction.Right)
            break
            case 'ArrowLeft':
            case 'KeyA':
                board.change_snake_dir(Direction.Left)
            break
        }
    })

    const drawBoard = () => {
        context.beginPath()

        for (let x = 0; x <= boardWidth; x++) {
            context.moveTo(CELL_SIZE * x, 0)
            context.lineTo(CELL_SIZE * x, boardWidth * CELL_SIZE)
        }

        for (let y = 0; y <= boardWidth; y++) {
            context.moveTo(0, CELL_SIZE * y,)
            context.lineTo(boardWidth * CELL_SIZE, CELL_SIZE * y)
        }

        context.stroke()
    }

    const drawRat = () => {
        const position = board.get_rat()
        const col = position % boardWidth
        const row = Math.floor(position / boardWidth)

        context.beginPath()
        context.fillStyle = '#808080'
        context.fillRect(
            col * CELL_SIZE,
            row * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE
        )
        context.stroke()
    }

    const drawSnake = () => {
        const snakeParts = new Uint32Array(wasm.memory.buffer, board.snake_parts(), board.snake_len())

        snakeParts
            .filter((p, i) => !(i > 0 && p === snakeParts[0])) 
            .forEach((p, i) => {
                const col = p % boardWidth
                const row = Math.floor(p / boardWidth)
        
                context.fillStyle = i === 0 ? '#dea584' : '#3178c6'
                context.beginPath()
                context.fillRect(
                    col * CELL_SIZE,
                    row * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE
                )
            })

        context.stroke()
    }

    const showGameStatus = () => {
        gameStatus.textContent = board.game_status_text()
        gamePoints.textContent = `${board.get_points()}`
    }

    const draw = () => {
        drawBoard()
        drawSnake()
        drawRat()
        showGameStatus()
    }

    const play = () => {
        if (board.game_status() == GameStatus.Win || board.game_status() == GameStatus.Loose) {
            playBtn.textContent = 'Qaytadan boshlash'
            return
        }

        const steps = 10
        setTimeout(() => {
            context.clearRect(0, 0, canvas.width, canvas.height)
            board.update()
            draw()
            requestAnimationFrame(play)
        }, 1000 / steps)
    }

    draw()
}

start()