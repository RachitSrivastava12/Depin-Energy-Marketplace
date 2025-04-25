import prisma from "@repo/db";
import { PublicKey } from "@solana/web3.js";
import { Router, type Request, type Response } from "express";


export const energylistRoute = Router();

energylistRoute.post('/create-energy-listing', async (req: Request, res: Response) => {
    console.log("reached inside create energy listing")
    const {pricePerKwh, energyAmount, sellerAddress} = req.body;
    console.log(pricePerKwh, energyAmount, sellerAddress, "from body")

    if(!pricePerKwh || !energyAmount || !sellerAddress) {
        res.status(400).json({ error: "Missing required fields" });
        return;
    }

    try {
        const user = await prisma .user.findUnique({
            where: {
                walletAddress: sellerAddress
            }
        })

        if (!user) {
            res.status(404).json({ error: "User not found" });
            return;
        }

        const energyListing = await prisma.energyListing.create({
            data: {
                pricePerKWh: parseFloat(pricePerKwh),
                amountKWh: parseFloat(energyAmount),
                sellerId: user.id
            },
        });

        res.status(201).json({ message: "Energy listing created successfully", energyListing });
    } catch (error) {
        console.error("Error creating energy listing:", error);
        res.status(500).json({ error: "Internal server error" });
        return;
    }

})