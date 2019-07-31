import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

ResponseObject response = WS.sendRequest(findTestObject('phs-transaction/checkDocId', [('endpoint') : GlobalVariable.endpoint
            , ('trxId') : findTestData('phs-transaction/checkDocId').getValue(1, 1), ('genId') : findTestData('phs-transaction/checkDocId').getValue(
                2, 1), ('policyNo') : findTestData('phs-transaction/checkDocId').getValue(3, 1), ('spajNo') : findTestData(
                'phs-transaction/checkDocId').getValue(4, 1), ('docIdType') : findTestData('phs-transaction/checkDocId').getValue(
                5, 1), ('statusCm') : findTestData('phs-transaction/checkDocId').getValue(6, 1), ('statusBpm') : findTestData(
                'phs-transaction/checkDocId').getValue(7, 1), ('statusDatacap') : findTestData('phs-transaction/checkDocId').getValue(
                8, 1), ('scanTimestamp') : findTestData('phs-transaction/checkDocId').getValue(9, 1), ('count') : findTestData(
                'phs-transaction/checkDocId').getValue(10, 1), ('alterNumber') : findTestData('phs-transaction/checkDocId').getValue(
                11, 1), ('trxIn') : findTestData('phs-transaction/checkDocId').getValue(12, 1), ('transactionType') : findTestData(
                'phs-transaction/checkDocId').getValue(13, 1), ('statusProcess') : findTestData('phs-transaction/checkDocId').getValue(
                14, 1), ('statusDelete') : findTestData('phs-transaction/checkDocId').getValue(15, 1), ('alterDate') : findTestData(
                'phs-transaction/checkDocId').getValue(16, 1), ('statusKtg') : findTestData('phs-transaction/checkDocId').getValue(
                17, 1), ('clientNoOwner') : findTestData('phs-transaction/checkDocId').getValue(18, 1), ('isUnitLink') : findTestData(
                'phs-transaction/checkDocId').getValue(19, 1), ('verify') : findTestData('phs-transaction/checkDocId').getValue(
                20, 1)]))

WS.verifyResponseStatusCode(response, 200)

