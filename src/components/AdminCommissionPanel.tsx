import { useState } from "react";
import { useStellarAccounts } from "../providers/StellarAccountProvider";
import { stellarService } from "../services/stellar.service";

export const AdminCommissionPanel = () => {
  const { walletAddress, setHashId } = useStellarAccounts();
  const [commissionAmount, setCommissionAmount] = useState<string>("");
  const [withdrawAmount, setWithdrawAmount] = useState<string>("");
  const [isLoading, setIsLoading] = useState(false);

  const handleSetCommission = async () => {
    if (!commissionAmount || parseFloat(commissionAmount) < 0) {
      alert("Please enter a valid commission amount");
      return;
    }

    setIsLoading(true);
    try {
      const txHash = await stellarService.setCommission(
        walletAddress,
        parseFloat(commissionAmount)
      );
      setHashId(txHash as string);
      alert("Commission set successfully!");
      setCommissionAmount("");
    } catch (error) {
      console.error("Error setting commission:", error);
      alert("Failed to set commission");
    } finally {
      setIsLoading(false);
    }
  };

  const handleWithdrawCommission = async () => {
    if (!withdrawAmount || parseFloat(withdrawAmount) <= 0) {
      alert("Please enter a valid amount to withdraw");
      return;
    }

    setIsLoading(true);
    try {
      const txHash = await stellarService.withdrawCommission(
        walletAddress,
        parseFloat(withdrawAmount)
      );
      setHashId(txHash as string);
      alert("Commission withdrawn successfully!");
      setWithdrawAmount("");
    } catch (error) {
      console.error("Error withdrawing commission:", error);
      alert("Failed to withdraw commission");
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="bg-white rounded-lg shadow-lg p-6 mb-6">
      <h2 className="text-xl font-bold mb-4 text-gray-800">
        Admin Commission Panel
      </h2>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* Set Commission */}
        <div className="border rounded-lg p-4">
          <h3 className="font-semibold text-gray-700 mb-3">Set Commission</h3>
          <div className="space-y-3">
            <input
              type="number"
              placeholder="Commission amount (XLM)"
              value={commissionAmount}
              onChange={(e) => setCommissionAmount(e.target.value)}
              className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
              disabled={isLoading}
            />
            <button
              onClick={() => void handleSetCommission()}
              disabled={isLoading || !commissionAmount}
              className="w-full px-4 py-2 bg-indigo-600 text-white font-semibold rounded-lg hover:bg-indigo-700 disabled:bg-gray-300 disabled:cursor-not-allowed transition-colors"
            >
              {isLoading ? "Processing..." : "Set Commission"}
            </button>
          </div>
        </div>

        {/* Withdraw Commission */}
        <div className="border rounded-lg p-4">
          <h3 className="font-semibold text-gray-700 mb-3">
            Withdraw Commission
          </h3>
          <div className="space-y-3">
            <input
              type="number"
              placeholder="Amount to withdraw (XLM)"
              value={withdrawAmount}
              onChange={(e) => setWithdrawAmount(e.target.value)}
              className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-transparent"
              disabled={isLoading}
            />
            <button
              onClick={() => void handleWithdrawCommission()}
              disabled={isLoading || !withdrawAmount}
              className="w-full px-4 py-2 bg-green-600 text-white font-semibold rounded-lg hover:bg-green-700 disabled:bg-gray-300 disabled:cursor-not-allowed transition-colors"
            >
              {isLoading ? "Processing..." : "Withdraw"}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};