import express, { type Request, type Response } from "express"
import { userRoute } from "./routes/user.router"

const app = express()

app.use(express.json())

const port = process.env.PORT || 8080


app.use('/api/v1', userRoute)

app.listen(port, () => {
    console.log(`Listening at port: ${port}`)
})