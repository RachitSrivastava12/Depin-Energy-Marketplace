import prisma from "@repo/db";
import { PublicKey } from "@solana/web3.js";
import { Router, type Request, type Response } from "express";


export const userRoute = Router();

userRoute.post('/connect-wallet', async (req: Request, res: Response) => {
    const { walletAddress } = req.body;
    console.log(walletAddress, "from body")

    if (!walletAddress) {
        res.status(401).json({ message: "Wallet address required" });
        return;
    }

    let isValidWalletAddress;

    try {
        isValidWalletAddress = new PublicKey(walletAddress);
        
    } catch (error) {
        res.json({ message: "Not a valid wallet address" })
    }

    try {
        if (isValidWalletAddress) {
            console.log("reached inside is valid")
            let user = await prisma.user.findFirst({
                where: {
                    walletAddress: walletAddress
                }
            })

            if (!user) {
                user = await prisma.user.create({
                    data: {
                        walletAddress,
                        airdropGiven: true
                    }
                })
            }

            res.status(200).send({ json: { message: "Wallet connected", user } })
        }
    } catch (error) {
        console.error(error);
        res.status(500).json({ message: "Error connecting wallet" });
    }
})