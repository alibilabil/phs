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
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

ResponseObject response =WS.sendRequest(findTestObject('phs-transaction/saveReportTransaction', [('endpoint') : GlobalVariable.endpoint, ('no_spajt') : findTestData(
                'phs-transaction/saveReportTransaction').getValue(1, 1), ('genId') : findTestData('phs-transaction/saveReportTransaction').getValue(
                2, 1), ('needDedup') : findTestData('phs-transaction/saveReportTransaction').getValue(3, 1), ('vendorName') : findTestData(
                'phs-transaction/saveReportTransaction').getValue(4, 1), ('add_new_life') : findTestData('phs-transaction/saveReportTransaction').getValue(
                5, 1), ('status') : findTestData('phs-transaction/saveReportTransaction').getValue(6, 1), ('manual_get_client') : findTestData(
                'phs-transaction/saveReportTransaction').getValue(7, 1), ('statusCase') : findTestData('phs-transaction/saveReportTransaction').getValue(
                8, 1), ('docIdType') : findTestData('phs-transaction/saveReportTransaction').getValue(9, 1), ('no_payor') : findTestData(
                'phs-transaction/saveReportTransaction').getValue(10, 1), ('policyNo') : findTestData('phs-transaction/saveReportTransaction').getValue(
                11, 1), ('needMagnum') : findTestData('phs-transaction/saveReportTransaction').getValue(12, 1)]))

WS.verifyResponseStatusCode(response, 200)