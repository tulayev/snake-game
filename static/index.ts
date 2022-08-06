import init, { Board } from 'snake-game'

const start = async () => {
    await init()

    const CELL_SIZE = 50
    const BOARD_WIDTH = 16
    const spawnIndex = Date.now() % (BOARD_WIDTH * BOARD_WIDTH)
    const board = Board.new(BOARD_WIDTH, spawnIndex)
    const boardWidth = board.width()

    const canvas = <HTMLCanvasElement>document.getElementById('snakeCanvas')
    const context = canvas.getContext('2d')
    
    canvas.width = canvas.height = boardWidth * CELL_SIZE
    
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

    const drawSnake = () => {
        const snakeIndex = board.snake_head_index()
        const col = snakeIndex % boardWidth
        const row = Math.floor(snakeIndex / boardWidth)

        context.beginPath()

        context.fillRect(
            col * CELL_SIZE,
            row * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE
        )

        context.stroke()
    }

    const draw = () => {
        drawBoard()
        drawSnake()
    }

    const update = () => {
        const fps = 10
        setTimeout(() => {
            context.clearRect(0, 0, canvas.width, canvas.height)
            board.update()
            draw()
            requestAnimationFrame(update)
        }, 1000 / fps)
    }

    draw()
    update()
}

start()