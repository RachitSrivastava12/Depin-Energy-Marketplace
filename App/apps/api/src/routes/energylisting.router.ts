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

        // Call the smart contract to create listing

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

energylistRoute.get('/get-energy-listings', async (req: Request, res: Response) => {
    console.log("reached inside get energy listing")
    try {
        const energyListings = await prisma.energyListing.findMany({
            include: {
                seller: true,
            },
        });

        res.status(200).json({ message: "Energy listings fetched successfully", energyListings });
    } catch (error) {
        console.error("Error fetching energy listings:", error);
        res.status(500).json({ error: "Internal server error" });
    }
})

energylistRoute.put('/update-energy-listing/:id', async (req: Request, res: Response) => {
    console.log("reached inside update energy listing")
    const { id } = req.params;
    const { pricePerKwh, energyAmount } = req.body;

    if (!pricePerKwh || !energyAmount) {
        res.status(400).json({ error: "Missing required fields" });
        return;
    }

    try {

        // Call the smart contract to update listing


        const energyListing = await prisma.energyListing.update({
            where: {
                id: id,
            },
            data: {
                pricePerKWh: parseFloat(pricePerKwh),
                amountKWh: parseFloat(energyAmount),
            },
        });

        res.status(200).json({ message: "Energy listing updated successfully", energyListing });
    } catch (error) {
        console.error("Error updating energy listing:", error);
        res.status(500).json({ error: "Internal server error" });
    }
}
)

energylistRoute.delete('/delete-energy-listing/:id', async (req: Request, res: Response) => {
    console.log("reached inside delete energy listing")
    const { id } = req.params;
    console.log(id, "from body")

    try {
        // Call the smart contract to delete listing
        await prisma.energyListing.delete({
            where: {
                id: id,
            },
        });

        res.status(200).json({ message: "Energy listing deleted successfully" });
    } catch (error) {
        console.error("Error deleting energy listing:", error);
        res.status(500).json({ error: "Internal server error" });
    }
})