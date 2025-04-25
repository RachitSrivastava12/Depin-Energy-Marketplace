import express, { type Request, type Response } from "express"
import { userRoute } from "./routes/user.router"
import { energylistRoute } from "./routes/energylisting.router"

const app = express()

app.use(express.json())

const port = process.env.PORT || 8080


app.use('/api/v1', userRoute)
app.use('/api/v1', energylistRoute)

app.listen(port, () => {
    console.log(`Listening at port: ${port}`)
})